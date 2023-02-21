<script lang="ts">
	import { getContext } from 'svelte';
	import { key, L, type MapContext } from '~/components/map3/leaflet';
	import {
		activeFeatureStoreArray,
		activeZone,
		isZoneFeature
	} from '~/components/map4/watershedStore';
	import { asgstr, sgstr, isEditingZone } from '~/components/mapStore';
	import { segmentStoreArray } from './watershedStore';

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
			return {
				color: feature.properties.color,
				weight: feature.properties.weight,
				fillOpacity: feature.properties.fillOpacity,
				pane: 'zone'
			};
		}
	};
	controller.addTo(map);

	$: if ($segmentStoreArray) {
		controller.clearLayers();

		for (const segment of $segmentStoreArray) {
			if (segment.properties.kind === 'SegmentZone') {
				controller.addData(segment);
			}
		}
	}
</script>
