<script lang="ts">
	import { onMount, setContext } from 'svelte';
	import { L, key, createMap } from '~/components/map3/leaflet';

	let element: HTMLDivElement;
	let map: L.Map;

	setContext(key, {
		getMap: () => map
	});

	onMount(() => {
		map = createMap(element);
	});
</script>

<div style="height:100%;width:100%" bind:this={element} />

{#if map}
	<slot />
{/if}
