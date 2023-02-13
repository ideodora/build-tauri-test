<script lang="ts">
	import L from 'leaflet';
	import { lineString, point } from '@turf/helpers';
	import { startIcon, endIcon } from '~/components/Map.svelte';
	import { onMount, createEventDispatcher } from 'svelte';
	import { sgstr } from '~/components/mapStore';

	const dispatch = createEventDispatcher();

	export let segment: any;

	const line = lineString(segment.segments, {
		id: `${segment.curveId}:line`,
		layerId: undefined,
		kind: 'SegmentLine' as const,
		weight: 5,
		color: '#0000FF'
	});
	const start = point(segment.segments[0], {
		id: `${segment.curveId}:start`,
		layerId: undefined,
		kind: 'SegmentStart' as const,
		icon: startIcon(),
		zIndexOffset: 1000
	});
	const end = point(segment.segments[segment.segments.length - 1], {
		id: `${segment.curveId}:end`,
		layerId: undefined,
		kind: 'SegmentEnd' as const,
		icon: endIcon(),
		zIndexOffset: 2000
	});
	const features = [line, start, end];
	sgstr.add(segment.curveId, {
		id: segment.curveId,
		features,
		line,
		start,
		end
	});

	// this.segments[id] = {
	// 	id,
	// 	features,
	// 	line,
	// 	start,
	// 	end
	// };
	// this.lib.addGeoJson(id, features);
	// this.lib.addGeoJson(id);
	// this.lib.addDataToGeoJson(id, line);
	// this.lib.addDataToGeoJson(id, start);
	// this.lib.addDataToGeoJson(id, end);
	// this.lib.plotGeoJson(id);
	// return features;

	// let id = `${startingPoint.lat}${startingPoint.lng}`;

	// let pointFeature = point([startingPoint.lng, startingPoint.lat], {
	// 	id,
	// 	pid: startingPoint.id,
	// 	kind: 'StartingPoint',
	// 	icon: iconDefault(),
	// 	zIndexOffset: 3000,
	// 	riverName: startingPoint.river_name,
	// 	riverCode: startingPoint.river_code,
	// 	prefCode: startingPoint.prefecture_code
	// });

	// let geoJson: L.GeoJSON = L.geoJSON(pointFeature, {
	// 	onEachFeature: (feature, layer) => {
	// 		layer.bindTooltip(`${feature.properties.riverName} (${feature.properties.riverCode})`);
	// 		layer.on('click', (ev) => {
	// 			console.log(ev);
	// 			console.log(layer);
	// 			console.log(feature);
	// 			dispatch('clickedStartingPoint', { feature, layer });
	// 		});
	// 	},
	// 	pointToLayer: function (feature, latlng) {
	// 		const marker = L.marker(latlng, {
	// 			icon: feature.properties.icon,
	// 			zIndexOffset: feature.properties.zIndexOffset
	// 		});
	// 		return marker;
	// 	}
	// });

	// const geoJson = L.geoJSON(features, {
	// 	onEachFeature: (feature, layer) => {
	// 		// this.geoJsonLayers[feature.properties.id] = layer;
	// 		if (layer instanceof L.Path) {
	// 			layer.on('pm:disable', (e) => {
	// 				// const latLngs = (e.layer as any).getLatLngs();
	// 				// const coords = L.GeoJSON.latLngsToCoords(latLngs, 1);
	// 				// const shape = (e.layer as L.Path).pm.getShape();
	// 				// if (shape === 'Polygon') {
	// 				// 	coords.forEach((sets) => {
	// 				// 		sets.push(sets[0]);
	// 				// 	});
	// 				// }
	// 				// const event = {
	// 				// 	feature: (e.layer as L.Polyline).feature,
	// 				// 	newCoords: coords
	// 				// };
	// 				// this.editedLayer$$.next(event);
	// 				console.log(e);
	// 			});
	// 		}
	// 	},
	// 	style: (feature) => {
	// 		if (!feature) {
	// 			return {};
	// 		}
	// 		const kind = feature.properties.kind;
	// 		switch (kind) {
	// 			case 'SegmentLine':
	// 				return {
	// 					color: feature.properties.color,
	// 					weight: feature.properties.weight
	// 				};
	// 			default:
	// 				return {};
	// 		}
	// 	},
	// 	pointToLayer: function (feature, latlng) {
	// 		const marker = L.marker(latlng, {
	// 			icon: feature.properties.icon,
	// 			zIndexOffset: feature.properties.zIndexOffset
	// 		});
	// 		return marker;
	// 	}
	// });

	// const geoJson = L.geoJson();
	// geoJson.options = {
	// 	onEachFeature: (feature, layer) => {
	// 		const layerId = geoJson.getLayerId(layer);
	// 		console.log('layerId:', layerId);
	// 		feature.properties.layerId = layerId;
	// 		// this.geoJsonLayers[feature.properties.id] = layer;
	// 		if (layer instanceof L.Path) {
	// 			layer.on('pm:disable', (e) => {
	// 				// const latLngs = (e.layer as any).getLatLngs();
	// 				// const coords = L.GeoJSON.latLngsToCoords(latLngs, 1);
	// 				// const shape = (e.layer as L.Path).pm.getShape();
	// 				// if (shape === 'Polygon') {
	// 				// 	coords.forEach((sets) => {
	// 				// 		sets.push(sets[0]);
	// 				// 	});
	// 				// }
	// 				// const event = {
	// 				// 	feature: (e.layer as L.Polyline).feature,
	// 				// 	newCoords: coords
	// 				// };
	// 				// this.editedLayer$$.next(event);
	// 				console.log(e);
	// 			});
	// 		}
	// 	},
	// 	style: (feature) => {
	// 		if (!feature) {
	// 			return {};
	// 		}
	// 		const kind = feature.properties.kind;
	// 		switch (kind) {
	// 			case 'SegmentLine':
	// 				return {
	// 					color: feature.properties.color,
	// 					weight: feature.properties.weight
	// 				};
	// 			default:
	// 				return {};
	// 		}
	// 	},
	// 	pointToLayer: function (feature, latlng) {
	// 		const marker = L.marker(latlng, {
	// 			icon: feature.properties.icon,
	// 			zIndexOffset: feature.properties.zIndexOffset
	// 		});
	// 		return marker;
	// 	}
	// };
	// for (const f of features) {
	// 	geoJson.addData(f);
	// }

	// const theFeature = $sgstr.get(segment.curveId);
	// console.log('theFeature', theFeature);

	// geoJson.on('click', (ev) => {
	// 	this.clickedGeoJson$$.next(id);
	// });
	// this.geoJsons[id] = geoJson;

	onMount(() => {
		dispatch('madeGeoJson', { features });
		return () => {
			// sgstr.remove(segment.curveId);
			dispatch('removeGeoJson', { features });
		};
	});
</script>
