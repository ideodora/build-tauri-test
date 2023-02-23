<script lang="ts">
	import SegmentMenu from '~/components/watershed-menu/segment-menu.svelte';
	import WatershedHeading from '~/components/watershed-menu/watershed-heading.svelte';
	import ZoneMenu from '~/components/watershed-menu/zone-menu.svelte';
	import { activeWatershedId, store } from './store';

	export let index: number;
	export let watershed: {
		id: number;
		name: string;
		children: any[];
		segments: {
			id: number;
			watershed_id: number;
			key: string;
			name: string;
		}[];
		zones: {
			id: number;
			watershed_id: number;
			key: string;
			name: string;
		}[];
	};

	const onClickWatershedKeydown = (e: KeyboardEvent, id: number) => {
		if (e.key === 'Enter') {
			onClickWatershed(id);
		}
	};
	const onClickWatershed = (id: number) => {
		$activeWatershedId = id;
	};
</script>

<li
	on:keydown={(e) => onClickWatershedKeydown(e, watershed.id)}
	on:click={() => onClickWatershed(watershed.id)}
>
	<details open={$store.get(`${index}`)}>
		<WatershedHeading {index} {watershed} on:deleteWatershed on:editWatershed />
		{#if watershed.children.length > 0}
			<ul>
				{#if watershed.segments.length > 0}
					<SegmentMenu parentIndex={index} index={0} segments={watershed.segments} />
				{/if}
				{#if watershed.zones.length > 0}
					<ZoneMenu parentIndex={index} index={1} zones={watershed.zones} />
				{/if}
			</ul>
		{/if}
	</details>
</li>
