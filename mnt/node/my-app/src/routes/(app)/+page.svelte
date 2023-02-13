<script lang="ts">
	import Map2 from '~/components/Map2.svelte';
	import MapContainer from '~/components/map3/MapContainer.svelte';
	import Navigation from '~/components/Navigation.svelte';
	import ImportModal from '~/components/ImportModal.svelte';
</script>

<ImportModal />

<div class="grid h-full w-full grid-rows-[auto,_min-content]">
	<MapContainer />
	<Navigation />
</div>

<style>
	:global(.firebase-emulator-warning) {
		display: none;
	}
</style>
