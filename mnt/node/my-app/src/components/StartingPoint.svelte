<script lang="ts">
	import L from 'leaflet';
	import { point } from '@turf/helpers';
	import { iconDefault } from '~/components/Map.svelte';
	import { onMount, createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let startingPoint: any;

	let id = `${startingPoint.lat}${startingPoint.lng}`;

	let pointFeature = point([startingPoint.lng, startingPoint.lat], {
		id,
		pid: startingPoint.id,
		kind: 'StartingPoint',
		icon: iconDefault(),
		zIndexOffset: 3000,
		riverName: startingPoint.river_name,
		riverCode: startingPoint.river_code,
		prefCode: startingPoint.prefecture_code
	});

	let geoJson = L.geoJSON(pointFeature, {
		onEachFeature: (feature, layer) => {
			layer.bindTooltip(`${feature.properties.riverName} (${feature.properties.riverCode})`);
			layer.on('click', (ev) => {
				console.log(ev);
				console.log(layer);
				console.log(feature);
				dispatch('clickedStartingPoint', { feature, layer });
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

	onMount(() => {
		dispatch('madeGeoJson', { geoJson });
		return () => {
			dispatch('removeGeoJson', { geoJson });
		};
	});
</script>
