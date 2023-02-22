<script lang="ts">
	import { getContext } from 'svelte';
	import { endIcon, key, L, startIcon, type MapContext } from '~/components/map4/leaflet';
	import { segmentsBounds } from '~/components/mapStore';
	import { featureStoreArray, activeSegment, activeFeatureStoreArray } from './watershedStore';

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
					if (!selectable()) return;
					const segmentId: string = feature.properties.id.split(':').slice(0, -1).join(':');
					if (e.originalEvent.shiftKey) {
						activeSegment.add(segmentId);
					} else {
						activeSegment.set(segmentId);
					}
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

	export const getBounds = () => {
		return controller.getBounds();
	};

	const activeController = L.geoJson();
	activeController.options = {
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
					const segmentId: string = feature.properties.id.split(':').slice(0, -1).join(':');

					if (e.originalEvent.shiftKey) {
						activeSegment.remove(segmentId);
					} else {
						activeSegment.set(segmentId);
					}
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
						color: '#ef4444',
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
	activeController.addTo(map);

	$: if ($featureStoreArray) {
		controller.clearLayers();

		for (const feature of $featureStoreArray) {
			if (feature.kind === 'SegmentFeature') {
				feature.start.properties.icon = startIcon();
				feature.end.properties.icon = endIcon();
				controller.addData(feature.line);
				controller.addData(feature.start);
				controller.addData(feature.end);
			}
		}

		if (controller.getLayers().length > 0) {
			const bounds = controller.getBounds();
			$segmentsBounds = bounds;
		}
	}

	$: if ($activeFeatureStoreArray) {
		activeController.clearLayers();

		for (const feature of $activeFeatureStoreArray) {
			if (feature.kind === 'SegmentFeature') {
				feature.start.properties.icon = startIcon();
				feature.end.properties.icon = endIcon();
				activeController.addData(feature.line);
				activeController.addData(feature.start);
				activeController.addData(feature.end);
			}
		}
	}
</script>
