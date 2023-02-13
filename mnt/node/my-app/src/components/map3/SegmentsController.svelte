<script lang="ts">
	import { getContext } from 'svelte';
	import { L, key, type MapContext } from '~/components/map3/leaflet';

	const { getMap } = getContext<MapContext>(key);

	const map = getMap();

	const controller = L.geoJson();
	controller.addTo(map);

	export const loadCurve = (feature: any) => {
		console.log('SegmentsController.loadCurve()', feature);
	};
</script>
