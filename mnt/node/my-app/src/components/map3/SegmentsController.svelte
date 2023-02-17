<script lang="ts">
	import { getContext } from 'svelte';
	import {
		L,
		key,
		startIcon,
		endIcon,
		startIconBig,
		endIconBig,
		type MapContext,
		type LassoHandlerFinishedEvent,
		type FeatureProperty,
		type FeaturePropertySegmentZone
	} from '~/components/map3/leaflet';
	import { lineString, point as turfPoint } from '@turf/helpers';
	import { default as turfClone } from '@turf/clone';
	import {
		sgstr,
		asgstr,
		drawingEnabled,
		isComposingZone,
		isEditingZone
	} from '~/components/mapStore';
	import { loadCurve as _loadCurve, loadCurveByKey as _loadCurveByKey } from './loadCurve';
	import { nanoid } from 'nanoid';
	import { default as turfBuffer } from '@turf/buffer';
	import { default as turfSimplify } from '@turf/simplify';
	import tokml from 'tokml';
	import { save as dialogSave } from '@tauri-apps/api/dialog';
	import { writeTextFile } from '@tauri-apps/api/fs';
	import { invoke } from '@tauri-apps/api/tauri';

	const { getMap } = getContext<MapContext>(key);

	const map = getMap();

	const controller = L.geoJson();
	controller.options = {
		onEachFeature: (feature, layer) => {
			const layerId = controller.getLayerId(layer);
			feature.properties.layerId = layerId;
			// this.geoJsonLayers[feature.properties.id] = layer;
			if (layer instanceof L.Path) {
				layer.on('pm:disable', (e) => {
					// const latLngs = (e.layer as any).getLatLngs();
					// const coords = L.GeoJSON.latLngsToCoords(latLngs, 1);
					// const shape = (e.layer as L.Path).pm.getShape();
					// if (shape === 'Polygon') {
					// 	coords.forEach((sets) => {
					// 		sets.push(sets[0]);
					// 	});
					// }
					// const event = {
					// 	feature: (e.layer as L.Polyline).feature,
					// 	newCoords: coords
					// };
					// this.editedLayer$$.next(event);
					console.log(e);
				});

				layer.on('click', (e) => {
					const curveId: string = feature.properties.id.split(':')[0];
					const segment = $sgstr.get(curveId);

					if (e.originalEvent.shiftKey) {
						asgstr.add(curveId, segment);
					} else {
						asgstr.set(curveId, segment);
					}

					// this.activeSegment = segment;
					// this._mode = 'selecting';
				});
			}
		},
		style: (feature) => {
			if (!feature) {
				return {};
			}
			const kind = feature.properties.kind;
			switch (kind) {
				case 'SegmentLine':
					return {
						color: feature.properties.color,
						weight: feature.properties.weight
					};
				case 'SegmentZone':
					return {
						color: feature.properties.color,
						weight: feature.properties.weight,
						fillOpacity: feature.properties.fillOpacity,
						pane: 'zone'
					};
				default:
					return {};
			}
		},
		pointToLayer: function (feature, latlng) {
			const marker = L.marker(latlng, {
				icon: feature.properties.icon,
				zIndexOffset: feature.properties.zIndexOffset
			});
			return marker;
		}
	};
	controller.addTo(map);

	const tempController = L.geoJson();
	tempController.options = {
		onEachFeature: (feature, layer) => {
			const layerId = tempController.getLayerId(layer);
			feature.properties.layerId = layerId;
			// this.geoJsonLayers[feature.properties.id] = layer;
			if (layer instanceof L.Path) {
				layer.on('pm:disable', (e) => {
					// const latLngs = (e.layer as any).getLatLngs();
					// const coords = L.GeoJSON.latLngsToCoords(latLngs, 1);
					// const shape = (e.layer as L.Path).pm.getShape();
					// if (shape === 'Polygon') {
					// 	coords.forEach((sets) => {
					// 		sets.push(sets[0]);
					// 	});
					// }
					// const event = {
					// 	feature: (e.layer as L.Polyline).feature,
					// 	newCoords: coords
					// };
					// this.editedLayer$$.next(event);
					console.log(e);
				});
			}
		},
		style: (feature) => {
			if (!feature) {
				return {};
			}
			const kind = feature.properties.kind;
			switch (kind) {
				case 'SegmentLine':
					return {
						color: feature.properties.color,
						weight: feature.properties.weight
					};
				case 'SegmentZone':
					return {
						color: feature.properties.color,
						weight: feature.properties.weight,
						fillOpacity: feature.properties.fillOpacity,
						pane: 'tempZone'
					};
				default:
					return {};
			}
		}
	};
	tempController.addTo(map);

	export const loadCurve = async (feature: any) => {
		const segments = await _loadCurve(feature);
		addSegments(segments);
	};

	export const loadCurveByKey = async (curveKey: any) => {
		const segments = await _loadCurveByKey(curveKey);
		addSegments(segments);
	};

	export const onMergedLasso = async (ev: any) => {
		const event = ev.detail as LassoHandlerFinishedEvent;
		console.log('lasso.finished in SegmentMerger', event);

		const lassoShape = L.GeoJSON.latLngsToCoords(event.latLngs);
		const targetFeatures = event.layers.map((layer: any) => layer.feature);
		const customEvent = { lassoShape, targetFeatures };

		console.dir(customEvent);

		// when is merge MODE
		mergeSegments(customEvent.targetFeatures);
		// if continue do it again else end
		// if (this.continueMerging) {
		//   setTimeout(() => {
		//     this.startMergingSegments();
		//   }, 10);
		// } else {
		//   this.cancelMergingSegments();
		// }
	};

	export const removeSelectingSegments = () => {
		const curveIds = $asgstr.keys();
		for (const curveId of curveIds) {
			removeSegment(curveId);
			asgstr.remove(curveId);
		}
	};

	export const swapStartEndSelectingSegment = () => {
		for (const curveId of $asgstr.keys()) {
			swapStartEnd(curveId);
		}
	};

	export const buildZone = (num: number = 100) => {
		tempController.clearLayers();

		for (const seg of $asgstr.values()) {
			console.log('seg:', seg);
			console.log('segId:', seg.id);

			$isComposingZone = true;
			const zone = seg.zone;
			const line = seg.line;

			const buffered = turfBuffer(line, num, {
				units: 'meters'
			});
			const tolerance = num * 0.0000025; // 20m => 0.00005
			const simplified = turfSimplify(buffered, {
				tolerance
			}) as GeoJSON.Feature<GeoJSON.Polygon, FeaturePropertySegmentZone>;
			simplified.properties.id = `${seg.id}:zone`;
			simplified.properties.kind = 'SegmentZone';
			simplified.properties.color = '#00FF00';
			simplified.properties.fillOpacity = 0.3;
			simplified.properties.weight = 1;

			tempController.addData(simplified);

			// seg.zone = simplified;
			// if (zone) {
			// 	seg.features.pop();
			// }
			// seg.features.push(simplified);
			// controller.addData(simplified);
		}
	};

	export const fixedZone = () => {
		const layers = tempController.getLayers();
		const layer = layers[0];
		const feature = (layer as any).feature;

		feature.properties.color = '#0EA5E9';

		const curveId = feature.properties.id.split(':')[0];
		const segment = $sgstr.get(curveId);

		const orgZone = segment.zone;
		if (orgZone) {
			segment.features.pop();
		}
		segment.zone = feature;
		segment.features.push(feature);

		controller.addData(feature);
		$isComposingZone = false;
	};

	export const exportSegments = async (ev: any) => {
		console.log(ev.fileName);

		const path = await dialogSave({
			defaultPath: 'exports.zip',
			filters: [
				{
					name: 'Zip',
					extensions: ['zip']
				}
			]
		});
		if (path) {
			const mapper = new Map<string, string>([]);

			for (const seg of $asgstr.values()) {
				const lineKml = tokml(seg.line);
				mapper.set(convertIdToKmlFileName(seg.line.properties.id), lineKml);

				if (seg.zone) {
					const zoneKml = tokml(seg.zone);
					mapper.set(convertIdToKmlFileName(seg.zone.properties.id), zoneKml);
				}
			}

			// writeTextFile(path, 'abc');

			const body = {
				path,
				data: [...mapper.entries()]
			};

			await invoke('save_export', { body });
		}
	};

	const convertIdToKmlFileName = (id: string) => {
		const replaced = id.replace(':', '_');
		return `${replaced}.kml`;
	};

	const addSegments = (segments: any[]) => {
		for (const segment of segments) {
			addSegment(segment);
		}
	};

	const addSegment = (segment: any) => {
		if ($sgstr.has(segment.curveId)) {
			return;
		}
		const line = lineString(segment.segments, {
			id: `${segment.curveId}:line`,
			layerId: undefined,
			kind: 'SegmentLine' as const,
			weight: 5,
			color: '#0000FF'
		});
		const start = turfPoint(segment.segments[0], {
			id: `${segment.curveId}:start`,
			layerId: undefined,
			kind: 'SegmentStart' as const,
			icon: startIcon(),
			zIndexOffset: 1000
		});
		const end = turfPoint(segment.segments[segment.segments.length - 1], {
			id: `${segment.curveId}:end`,
			layerId: undefined,
			kind: 'SegmentEnd' as const,
			icon: endIcon(),
			zIndexOffset: 2000
		});
		const features = [line, start, end];

		controller.addData(line);
		controller.addData(start);
		controller.addData(end);

		sgstr.add(segment.curveId, {
			id: segment.curveId,
			features,
			line,
			start,
			end
		});
	};

	const removeSegment = (curveId: string) => {
		const segment = $sgstr.get(curveId);
		controller.removeLayer(segment.line.properties.layerId);
		controller.removeLayer(segment.start.properties.layerId);
		controller.removeLayer(segment.end.properties.layerId);
		if (segment.zone) {
			controller.removeLayer(segment.zone.properties.layerId);
		}
		sgstr.remove(curveId);
	};

	const swapStartEnd = (curveId: string) => {
		const segment = $sgstr.get(curveId);

		const id = nanoid();
		const newCurveId = `swapedLine-${id}`;

		const line = turfClone(segment.line);
		line.properties.id = `${newCurveId}:line`;
		line.properties.layerId = undefined;

		const start = turfClone(segment.end);
		start.properties.id = `${newCurveId}:start`;
		start.properties.layerId = undefined;
		start.properties.icon = startIcon();

		const end = turfClone(segment.start);
		end.properties.id = `${newCurveId}:end`;
		end.properties.layerId = undefined;
		end.properties.icon = endIcon();

		const features = [line, start, end];

		controller.addData(line);
		controller.addData(start);
		controller.addData(end);

		sgstr.add(newCurveId, {
			id: newCurveId,
			features,
			line,
			start,
			end
		});
		removeSegment(curveId);
		asgstr.reset();
	};

	const resetAllStyle = () => {
		controller.resetStyle();
		controller.eachLayer((layer) => {
			if (layer instanceof L.Marker) {
				layer.setIcon(layer.feature?.properties.icon);
				layer.setZIndexOffset(layer.feature?.properties.zIndexOffset);
			}
		});
	};

	const mergeSegments = (
		features: GeoJSON.Feature<
			GeoJSON.LineString | GeoJSON.Point | GeoJSON.Polygon,
			FeatureProperty
		>[]
	) => {
		// // confirmation
		// if (!this.continueMerging && !confirm('連結しますか?')) {
		// 	return;
		// }

		// extract starts and ends
		const starts = features.filter((feature) => feature.properties.kind === 'SegmentStart');
		const ends = features.filter((feature) => feature.properties.kind === 'SegmentEnd');

		// if not single pair error
		if (starts.length !== 1 || ends.length !== 1) {
			alert('no single conjunction');
			return;
		}

		// id extraction
		const formerId = ends[0].properties.id.split(':')[0];
		const latterId = starts[0].properties.id.split(':')[0];

		// get segment from store
		const former = $sgstr.get(formerId);
		const latter = $sgstr.get(latterId);

		// extract lines
		const formerLine = former.line;
		const latterLine = latter.line;

		// unite coodinates
		const formerCorrds = formerLine!.geometry.coordinates as GeoJSON.Position[];
		const latterCorrds = latterLine!.geometry.coordinates as GeoJSON.Position[];
		const merged = formerCorrds.concat(latterCorrds);

		// add to store with new id
		const id = nanoid();
		const segmentId = `mergedLine-${id}`;
		// // this.addSegment(segmentId, merged);
		const newSegment = { curveId: segmentId, segments: merged };
		addSegment(newSegment);
		removeSegment(formerId);
		removeSegment(latterId);

		// const mapper = new Map($storeSegments.map((item) => [item.curveId, item]));
		// mapper.set(segmentId, newSegment);
		// storeSegments.set([...mapper.values()]);

		// TODO:

		// // zone merging
		// const formerZone = former.zone;
		// const latterZone = latter.zone;
		// const zones = [formerZone, latterZone].filter((v) => v) as GeoJSON.Feature<
		// 	GeoJSON.Polygon,
		// 	FeaturePropertySegmentZone
		// >[];

		// TODO: merge zone
		// // if only 2 separated zones
		// if (zones.length === 2) {
		// 	const newZone = union(zones[0], zones[1]);
		// 	// only if single polygon not multipolygon
		// 	if (newZone?.geometry.type === 'Polygon') {
		// 		debugger;
		// 		const newZoneId = `${segmentId}:zone`;
		// 		const newFeature = polygon(newZone.geometry.coordinates, {
		// 			id: newZoneId,
		// 			kind: 'SegmentZone' as const,
		// 			color: '#00FF00',
		// 			weight: 1,
		// 			fillOpacity: 0.3
		// 		}) as GeoJSON.Feature<GeoJSON.Polygon, FeaturePropertySegmentZone>;
		// 		this.segments[segmentId].features.push(newFeature);
		// 		this.segments[segmentId].zone = newFeature;
		// 		this.lib.addDataToGeoJson(segmentId, newFeature);
		// 	}
		// }

		// // need outside function?
		// $segmentsStore.set(segmentId, merged);
		// $segmentsStore.delete(formerId);
		// $segmentsStore.delete(latterId);
		// // clone
		// $segmentsStore = new Map($segmentsStore);
	};

	map.on('pm:create', (e) => {
		const feature = (e.layer as L.Polyline<GeoJSON.LineString>).toGeoJSON();
		const random = nanoid();
		const id = `drawedSegment-${random}`;
		const newSegment = { curveId: id, segments: feature.geometry.coordinates };
		addSegment(newSegment);
		$drawingEnabled = false;
		map.removeLayer(e.layer);
	});

	$: {
		if ($asgstr.size === 0) {
			resetAllStyle();
		}

		if ($asgstr.size > 0) {
			resetAllStyle();
			for (const seg of $asgstr.values()) {
				const lineLayer = controller.getLayer(seg.line.properties.layerId);
				const startLayer = controller.getLayer(seg.start.properties.layerId);
				const endLayer = controller.getLayer(seg.end.properties.layerId);

				(lineLayer as L.Path)?.setStyle({
					color: '#FF0000'
				});
				(startLayer as L.Marker)?.setIcon(startIconBig()).setZIndexOffset(10000);
				(endLayer as L.Marker)?.setIcon(endIconBig()).setZIndexOffset(20000);
			}
		}

		if ($drawingEnabled) {
			map.pm.enableDraw('Line', {
				snappable: true,
				snapDistance: 20
			});
		} else {
			map.pm.disableDraw();
		}

		if (!$isComposingZone) {
			tempController.clearLayers();
		}

		if ($isEditingZone) {
			for (const seg of $asgstr.values()) {
				const zone = seg.zone;
				if (!zone) continue;

				const zoneLayer = controller.getLayer(zone.properties.layerId);
				if (!zoneLayer) continue;

				if (zoneLayer instanceof L.Path) {
					(zoneLayer as L.Path).pm.setOptions({
						limitMarkersToCount: 20,
						snapDistance: 5
					});
					(zoneLayer as L.Path).pm.enable();
				}
			}
		}

		if (!$isEditingZone) {
			for (const seg of $asgstr.values()) {
				const zone = seg.zone;
				if (!zone) continue;

				const zoneLayer = controller.getLayer(zone.properties.layerId);
				if (!zoneLayer) continue;

				if (zoneLayer instanceof L.Path) {
					(zoneLayer as L.Path).pm.disable();
				}
			}
		}
	}
</script>
