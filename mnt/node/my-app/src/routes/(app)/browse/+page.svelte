<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { confirm } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import EditSegmentPanel from '~/components/edit-segment-panel/index.svelte';
	import EditWatershedPanel from '~/components/edit-watershed-panel/index.svelte';
	import EditZonePanel from '~/components/edit-zone-panel/index.svelte';
	import MapComponent from '~/components/map4/MapComponent.svelte';
	import SegmentsProjection from '~/components/map4/SegmentsProjection.svelte';
	import ZoneProjection from '~/components/map4/ZoneProjection.svelte';
	import WatershedMenu from '~/components/watershed-menu/index.svelte';
	import {
		activeWatershed,
		activeWatershedId,
		offscreen,
		watershedMapStore
	} from '~/store/browseStore';
	import {
		featureStore,
		segmentVisibilityFilter,
		zoneVisibilityFilter
	} from '~/store/featureStore';

	beforeNavigate(() => {
		activeWatershedId.reset();
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
		watershedMapStore.fromArray(res.watersheds ?? []);
	};

	const onDeleteWatershed = async () => {
		const confirmed = await confirm('この作業は元に戻せません', {
			title: '本当に削除してよろしいですか',
			type: 'warning'
		});
		if (!confirmed) return;

		const id = $activeWatershedId;
		const res: any = await invoke('delete_watershed', { payload: { id } });
		watershedMapStore.remove(id);
		featureStore.reset();
		activeWatershedId.reset();
	};

	const onEditWatershed = async () => {
		$offscreen = 'watershed';
	};

	$: if ($activeWatershed) {
		$offscreen = undefined;
		featureStore.reset();

		const children = $activeWatershed.children.map((child: any) => {
			return { ...child, data: JSON.parse(child.data) };
		});

		for (const child of children) {
			featureStore.add(child.key, child.data);
		}
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
			class="absolute right-0 z-[1000] block h-full w-[21rem] border-l border-gray-300 bg-gray-50"
			transition:fly|local={{ x: 336, duration: 100 }}
		>
			{#if $offscreen === 'watershed'}
				<EditWatershedPanel />
			{:else if $offscreen === 'segment'}
				<EditSegmentPanel />
			{:else if $offscreen === 'zone'}
				<EditZonePanel />
			{/if}
		</div>
	{/if}
</div>
