<script context="module" lang="ts">
	console.log('map2 script module');
</script>

<script lang="ts">
	console.log('map2 script');
	import Map from '~/components/Map.svelte';
	import SearchBlock from '~/components/SearchBlock.svelte';
	import ToolBlock from '~/components/ToolBlock.svelte';

	let theMap: Map;

	const onClickStartingPoint = (event: any) => {
		console.log(event);
	};

	const onCreateZoneEvent = () => {
		theMap.createZone(100);
	};
</script>

<div class="relative h-full w-full">
	<div class="pointer-events-none absolute inset-0 z-20">
		<div class="pl-2 pt-2">
			<SearchBlock />
		</div>
		<div class="pl-2 pt-2">
			<ToolBlock on:createZoneEvent={onCreateZoneEvent} />
		</div>
	</div>
	<div class="absolute inset-0 z-10">
		<Map on:clickStartingPoint={onClickStartingPoint} bind:this={theMap} />
	</div>
</div>
