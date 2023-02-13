<script lang="ts">
	import { sps, sgstr } from '~/components/mapStore';
	import L from 'leaflet';
	export let map: L.Map;
	import StartingPoint from '~/components/StartingPoint.svelte';

	console.log('starting point cluster');

	const startingPointCluster = L.markerClusterGroup();

	console.log('starting point cluster:', map);
	startingPointCluster.addTo(map);

	const onMadeGeoJson = (event: any) => {
		const geoJson = event.detail.geoJson as L.GeoJSON;
		startingPointCluster.addLayer(geoJson);
	};

	const onRemoveGeoJson = (event: any) => {
		startingPointCluster.removeLayer(event.detail.geoJson);
	};
</script>

{#each $sps as sp (sp.id + 'n' + sp.nid + 's' + sp.sid + '@' + sp.prefecture_code)}
	<StartingPoint
		startingPoint={sp}
		on:madeGeoJson={onMadeGeoJson}
		on:removeGeoJson={onRemoveGeoJson}
		on:clickedStartingPoint
	/>
{/each}
