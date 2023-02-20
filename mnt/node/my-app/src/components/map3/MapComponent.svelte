<script lang="ts">
	import { createEventDispatcher, onMount, setContext } from 'svelte';
	import { L, key, createMap } from '~/components/map3/leaflet';

	const dispathcer = createEventDispatcher();

	let element: HTMLDivElement;
	let map: L.Map;

	setContext(key, {
		getMap: () => map
	});

	onMount(() => {
		map = createMap(element);

		// for zone layer zindex
		map.createPane('tempZone').style.zIndex = '250';
		map.createPane('zone').style.zIndex = '350';
		dispathcer('ready');
	});
</script>

<div style="height:100%;width:100%" bind:this={element} />

{#if map}
	<slot />
{/if}

<style lang="postcss">
	:global(.leaflet-overlay-pane) {
		-webkit-user-select: none; /* Safari */
		-ms-user-select: none; /* IE 10 and IE 11 */
		user-select: none; /* Standard syntax */
	}
</style>
