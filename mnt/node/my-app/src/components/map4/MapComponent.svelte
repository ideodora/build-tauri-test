<script lang="ts">
	import { createEventDispatcher, onMount, setContext } from 'svelte';
	import { L, key, createMap } from '~/components/map4/leaflet';
	import { focusBounds, syncCenter } from '~/store/mapStore';

	const dispathcer = createEventDispatcher();

	let element: HTMLDivElement;
	let map: L.Map;

	export let autoFocus: boolean = false;
	export let selectable: boolean = true;
	export let defaultCenter: L.LatLngExpression | undefined = undefined;

	setContext(key, {
		getMap: () => map,
		selectable: () => selectable
	});

	onMount(async () => {
		map = await createMap(element, defaultCenter);

		// for zone layer zindex
		map.createPane('tempZone').style.zIndex = '250';
		map.createPane('zone').style.zIndex = '350';
		dispathcer('ready');

		map.on('keydown', (ev) => {
			dispathcer('mapKeydown', ev);
		});
	});

	$: if ($focusBounds) {
		if (map && autoFocus) {
			map.fitBounds($focusBounds);
			$syncCenter = map.getCenter();
		}
	}
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
