<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import MapComponent from '~/components/map3/MapComponent.svelte';
	import SegmentsProjection from '~/components/map3/SegmentsProjection.svelte';
	import { segmentStore } from '~/components/map3/watershedStore';
	import ZoneProjection from '~/components/map3/ZoneProjection.svelte';
	import TreeMenu from '~/components/tree-menu/index.svelte';
	import { watershedStore } from '~/routes/(app)/browse/store';

	let segmentsProjection: SegmentsProjection;

	const onMapReady = async () => {
		const res: any = await invoke('watersheds');
		$watershedStore = res.watersheds ?? [];
	};

	const onClickedWatershed = (ev: any) => {
		const index = ev.detail.index;
		const watershed = $watershedStore[index];

		const children = watershed.children.map((child: any) => {
			return { ...child, data: JSON.parse(child.data) };
		});

		segmentStore.reset();

		for (const child of children) {
			segmentStore.add(child.data);
		}
	};
</script>

<div class="grid grid-cols-[25ch,_1fr]">
	<div class="border-r bg-neutral-100">
		<TreeMenu on:clickedWatershed={onClickedWatershed} />
	</div>
	<div class="bg-slate-100">
		<MapComponent on:ready={onMapReady}>
			<SegmentsProjection bind:this={segmentsProjection} />
			<ZoneProjection />
		</MapComponent>
	</div>
</div>
