<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { confirm } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import MapComponent from '~/components/map4/MapComponent.svelte';
	import SegmentsProjection from '~/components/map4/SegmentsProjection.svelte';
	import ZoneProjection from '~/components/map4/ZoneProjection.svelte';
	import WatershedMenu from '~/components/watershed-menu/index.svelte';

	import { onMount } from 'svelte';
	import {
		featureStore,
		segmentVisibility,
		segmentVisibilityFilter,
		zoneVisibility,
		zoneVisibilityFilter
	} from '~/components/map4/watershedStore';
	import { activeWatershedId } from '~/components/watershed-menu/store';
	import {
		activeWatershed,
		watershedStore,
		offscreen,
		editSegment,
		editZone
	} from '~/routes/(app)/browse/store';
	import { fly } from 'svelte/transition';
	import EditWatershedPanel from '~/components/edit-watershed-panel/index.svelte';
	import EditZonePanel from '~/components/edit-zone-panel/index.svelte';
	import EditSegmentPanel from '~/components/edit-segment-panel/index.svelte';

	beforeNavigate(() => {
		activeWatershedId.reset();
		$segmentVisibility = true;
		$zoneVisibility = true;
		segmentVisibilityFilter.reset();
		zoneVisibilityFilter.reset();
	});

	let segmentsProjection: SegmentsProjection;
	let zoneProjection: ZoneProjection;

	onMount(() => {
		featureStore.reset();
	});

	const onMapReady = async () => {
		const res: any = await invoke('watersheds');
		$watershedStore = res.watersheds ?? [];
	};

	const onDeleteWatershed = async () => {
		const confirmed = await confirm('この作業は元に戻せません', {
			title: '本当に削除してよろしいですか',
			type: 'warning'
		});
		if (!confirmed) return;

		const id = $activeWatershedId;
		const res: any = await invoke('delete_watershed', { payload: { id } });
		// TODO: remove from store;
		$watershedStore = $watershedStore.filter((watershed) => watershed.id !== id);
		featureStore.reset();
		activeWatershedId.reset();
	};

	const onEditWatershed = async () => {
		console.log('edit watershed');
		$offscreen = !$offscreen;
	};

	$: if ($activeWatershed) {
		$offscreen = false;
		const children = $activeWatershed.children.map((child: any) => {
			return { ...child, data: JSON.parse(child.data) };
		});

		featureStore.reset();

		for (const child of children) {
			featureStore.add(child.key, child.data);
		}

		// const segmentBounds = segmentsProjection.getBounds();
		// const zoneBounds = zoneProjection.getBounds();
		// const bounds = segmentBounds.extend(zoneBounds);
		// $focusBounds = bounds;
	}
</script>

<div class="relative grid h-full grid-cols-[25ch,_1fr] overflow-hidden">
	<div class="overflow-auto border-r bg-neutral-100">
		<WatershedMenu on:deleteWatershed={onDeleteWatershed} on:editWatershed={onEditWatershed} />
	</div>
	<div class="bg-slate-100">
		<MapComponent on:ready={onMapReady} autoFocus={true} selectable={false}>
			<SegmentsProjection bind:this={segmentsProjection} />
			<ZoneProjection bind:this={zoneProjection} />
		</MapComponent>
	</div>
	{#if $offscreen}
		<div
			class="absolute right-0 z-[1000] block h-full w-80 border-l border-gray-300 bg-gray-50"
			transition:fly|local={{ x: 320, duration: 100 }}
		>
			<EditWatershedPanel />
		</div>
	{/if}
	{#if $editSegment}
		<div
			class="absolute right-0 z-[1000] block h-full w-80 border-l border-gray-300 bg-gray-50"
			transition:fly|local={{ x: 320, duration: 100 }}
		>
			<EditSegmentPanel />
		</div>
	{/if}
	{#if $editZone}
		<div
			class="absolute right-0 z-[1000] block h-full w-80 border-l border-gray-300 bg-gray-50"
			transition:fly|local={{ x: 320, duration: 100 }}
		>
			<EditZonePanel />
		</div>
	{/if}
</div>
