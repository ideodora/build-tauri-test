<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { confirm } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import MapComponent from '~/components/map4/MapComponent.svelte';
	import SegmentsProjection from '~/components/map4/SegmentsProjection.svelte';
	import ZoneProjection from '~/components/map4/ZoneProjection.svelte';
	import TreeMenu from '~/components/tree-menu/index.svelte';

	import { onMount } from 'svelte';
	import {
		featureStore,
		segmentVisibility,
		segmentVisibilityFilter,
		zoneVisibility,
		zoneVisibilityFilter
	} from '~/components/map4/watershedStore';
	import { activeWatershedId } from '~/components/tree-menu/store';
	import { activeWatershed, watershedStore } from '~/routes/(app)/browse/store';

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

	const deleteWatershed = async () => {
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

	$: if ($activeWatershed) {
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

<div class="grid h-full grid-cols-[25ch,_1fr] overflow-hidden">
	<div class="overflow-auto border-r bg-neutral-100">
		<TreeMenu on:deleteWatershed={deleteWatershed} />
	</div>
	<div class="bg-slate-100">
		<MapComponent on:ready={onMapReady} autoFocus={true}>
			<SegmentsProjection bind:this={segmentsProjection} />
			<ZoneProjection bind:this={zoneProjection} />
		</MapComponent>
	</div>
</div>
