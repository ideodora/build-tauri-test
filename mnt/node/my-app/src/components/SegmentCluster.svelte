<script lang="ts">
	import L from 'leaflet';
	import { segments as storeSegments, sgstr, asgstr, zsgstr } from '~/components/mapStore';
	export let map;

	import Segment from '~/components/Segment.svelte';
	import { endIconBig, startIconBig } from '~/components/Map.svelte';
	import buffer from '@turf/buffer';
	import simplify from '@turf/simplify';

	const segmentCluster = L.geoJson();

	segmentCluster.options = {
		onEachFeature: (feature, layer) => {
			const layerId = segmentCluster.getLayerId(layer);
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
						pane: 'buffers'
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
	segmentCluster.addTo(map);

	type FeaturePropertySegmentZone = {
		id: string;
		kind: 'SegmentZone';
		fillOpacity: number;
		weight: number;
		color: string;
	};

	$: {
		if ($asgstr.size === 0) {
			resetAllStyle();
		}

		if ($asgstr.size > 0) {
			resetAllStyle();
			for (const seg of $asgstr.values()) {
				const lineLayer = segmentCluster.getLayer(seg.line.properties.layerId);
				const startLayer = segmentCluster.getLayer(seg.start.properties.layerId);
				const endLayer = segmentCluster.getLayer(seg.end.properties.layerId);

				(lineLayer as L.Path)?.setStyle({
					color: '#FF0000'
				});
				(startLayer as L.Marker)?.setIcon(startIconBig()).setZIndexOffset(10000);
				(endLayer as L.Marker)?.setIcon(endIconBig()).setZIndexOffset(20000);
			}
		}
	}

	export const createZone = (num: number) => {
		for (const seg of $asgstr.values()) {
			debugger;
			const zone = seg.zone;
			const line = seg.line;

			const buffered = buffer(line, num, {
				units: 'meters'
			});
			const tolerance = num * 0.0000025; // 20m => 0.00005
			const simplified = simplify(buffered, {
				tolerance
			}) as GeoJSON.Feature<GeoJSON.Polygon, FeaturePropertySegmentZone>;
			simplified.properties.id = `${seg.curveId}:zone`;
			simplified.properties.kind = 'SegmentZone';
			simplified.properties.color = '#00FF00';
			simplified.properties.fillOpacity = 0.3;
			simplified.properties.weight = 1;
			seg.zone = simplified;

			if (zone) {
				seg.features.pop();
			}
			seg.features.push(simplified);
			segmentCluster.addData(simplified);
		}
		console.log($asgstr);
	};

	const resetAllStyle = () => {
		segmentCluster.resetStyle();
		segmentCluster.eachLayer((layer) => {
			if (layer instanceof L.Marker) {
				layer.setIcon(layer.feature?.properties.icon);
				layer.setZIndexOffset(layer.feature?.properties.zIndexOffset);
			}
		});
	};

	const onMadeGeoJson = (event: any) => {
		const features = event.detail.features;
		for (const feature of features) {
			segmentCluster.addData(feature);
		}
	};

	const onRemoveGeoJson = (event: any) => {
		const features = event.detail.features;
		for (const feature of features) {
			segmentCluster.removeLayer(feature.properties.layerId);
		}
		// segmentCluster.removeLayer(event.detail.geoJson);
	};
</script>

{#each $storeSegments as storeSegment (storeSegment.curveId)}
	<Segment
		segment={storeSegment}
		on:madeGeoJson={onMadeGeoJson}
		on:removeGeoJson={onRemoveGeoJson}
	/>
{/each}

<style lang="postcss">
	:global(.leaflet-overlay-pane) {
		-webkit-user-select: none; /* Safari */
		-ms-user-select: none; /* IE 10 and IE 11 */
		user-select: none; /* Standard syntax */
	}
</style>
