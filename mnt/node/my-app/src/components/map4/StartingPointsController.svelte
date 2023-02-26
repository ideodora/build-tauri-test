<script lang="ts">
	import { getContext, onDestroy } from 'svelte';
	import { L, key, iconDefault, type MapContext } from '~/components/map4/leaflet';
	import { invoke } from '@tauri-apps/api/tauri';
	import { rawStartingPoints } from '~/store/mapStore';
	import { point as turfPoint } from '@turf/helpers';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	const { getMap } = getContext<MapContext>(key);

	const map = getMap();

	const controller = L.markerClusterGroup();
	controller.addTo(map);

	export const load = async () => {
		const bounds = map.getBounds();
		const params = {
			min: bounds.getSouthWest(),
			max: bounds.getNorthEast()
		};

		const startingPoints = await invoke<any>('search_points', { bounds: params });

		rawStartingPoints.set(startingPoints.points);
	};

	export const unload = async () => {
		rawStartingPoints.set([]);
	};

	const unsubscribe = rawStartingPoints.subscribe(async (points) => {
		controller.clearLayers();

		for (const point of points) {
			let id = `${point.lat}${point.lng}`;
			let pointFeature = turfPoint([point.lng, point.lat], {
				id,
				pid: point.id,
				kind: 'StartingPoint',
				icon: iconDefault(),
				zIndexOffset: 3000,
				riverName: point.river_name,
				riverCode: point.river_code,
				prefCode: point.prefecture_code
			});

			let geoJson = L.geoJSON(pointFeature, {
				onEachFeature: (feature, layer) => {
					layer.bindTooltip(`${feature.properties.riverName} (${feature.properties.riverCode})`);
					layer.on('click', () => {
						dispatch('clickedPoint', { feature });
					});
				},
				pointToLayer: function (feature, latlng) {
					const marker = L.marker(latlng, {
						icon: feature.properties.icon,
						zIndexOffset: feature.properties.zIndexOffset
					});
					return marker;
				}
			});

			controller.addLayer(geoJson);
		}
	});

	onDestroy(unsubscribe);
</script>
