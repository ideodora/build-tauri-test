<script lang="ts">
	import { getContext } from 'svelte';
	import { key, L, type MapContext } from '~/components/map4/leaflet';
	import { asgstr, sgstr, isEditingZone, zonesBounds } from '~/components/mapStore';
	import {
		activeZone,
		featureStoreArray,
		activeFeatureStoreArray,
		isZoneFeature
	} from './watershedStore';

	const { getMap } = getContext<MapContext>(key);
	const { selectable } = getContext<MapContext>(key);

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
			if (layer instanceof L.Polygon) {
				layer.on('mouseover', () => {
					layer.setStyle({
						fillOpacity: 0.4
					});
				});
				layer.on('mouseout', () => {
					layer.setStyle({
						fillOpacity: 0.3
					});
				});
				layer.on('click', (e) => {
					if (!selectable()) return;
					const featureId: string = feature.properties.id;
					if (e.originalEvent.shiftKey) {
						activeZone.add(featureId);
					} else {
						activeZone.set(featureId);
					}
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

	export const getBounds = () => {
		return controller.getBounds();
	};

	const activeController = L.geoJson();
	activeController.options = {
		onEachFeature: (feature, layer) => {
			const layerId = activeController.getLayerId(layer);
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
			if (layer instanceof L.Polygon) {
				console.log('instnace of zone');

				layer.on('pm:disable', (e) => {
					console.log('after edit zone');

					const latLngs = layer.getLatLngs();
					const coordSets = L.GeoJSON.latLngsToCoords(latLngs, 1);
					const shape = layer.pm.getShape();

					// make it closed polygon
					if (shape === 'Polygon') {
						coordSets.forEach((coords) => {
							coords.push(coords[0]);
						});
					}

					console.log('before set coordSets:', feature);

					(feature.geometry as any).coordinates = coordSets;

					console.log(e);
				});

				// layer.on('mouseover', () => {
				// 	console.log('mouseover');
				// 	layer.setStyle({
				// 		fillOpacity: 0.4
				// 	});
				// });
				// layer.on('mouseout', () => {
				// 	console.log('mouseout');
				// 	layer.setStyle({
				// 		fillOpacity: 0.3
				// 	});
				// });
				layer.on('click', (e) => {
					const featureId: string = feature.properties.id;
					console.log('click zone id:', featureId);

					if (e.originalEvent.shiftKey) {
						activeZone.remove(featureId);
					} else {
						activeZone.set(featureId);
					}
				});
			}
			console.log(layer);
		},
		style: (feature) => {
			if (!feature) {
				return {};
			}
			return {
				color: '#ef4444',
				weight: feature.properties.weight,
				fillOpacity: 0.3,
				pane: 'zone'
			};
		}
	};
	activeController.addTo(map);

	$: if ($featureStoreArray) {
		controller.clearLayers();

		for (const feature of $featureStoreArray) {
			if (feature.kind === 'ZoneFeature') {
				controller.addData(feature.zone);
			}
		}

		if (controller.getLayers().length > 0) {
			const bounds = controller.getBounds();
			$zonesBounds = bounds;
		}
	}

	$: if ($activeFeatureStoreArray) {
		activeController.clearLayers();

		for (const feature of $activeFeatureStoreArray) {
			if (feature.kind === 'ZoneFeature') {
				activeController.addData(feature.zone);
			}
		}
	}

	$: {
		if ($isEditingZone) {
			for (const feature of $activeFeatureStoreArray) {
				if (!isZoneFeature(feature)) continue;

				const layerId = feature.zone.properties.layerId!;
				const zoneLayer = activeController.getLayer(layerId);

				if (!zoneLayer) continue;

				if (zoneLayer instanceof L.Polygon) {
					(zoneLayer as L.Polygon).pm.setOptions({
						limitMarkersToCount: 20,
						snapDistance: 5
					});
					(zoneLayer as L.Polygon).pm.enable();
				}
			}
		}

		if (!$isEditingZone) {
			for (const feature of $activeFeatureStoreArray) {
				if (!isZoneFeature(feature)) continue;

				const layerId = feature.zone.properties.layerId!;
				const zoneLayer = activeController.getLayer(layerId);

				if (!zoneLayer) continue;

				if (zoneLayer instanceof L.Polygon) {
					(zoneLayer as L.Polygon).pm.disable();
				}
			}
		}
	}
</script>
