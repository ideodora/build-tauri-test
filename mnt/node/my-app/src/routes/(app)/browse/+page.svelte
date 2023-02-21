<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import MapComponent from '~/components/map4/MapComponent.svelte';
	import SegmentsProjection from '~/components/map4/SegmentsProjection.svelte';
	import ZoneProjection from '~/components/map4/ZoneProjection.svelte';
	import TreeMenu from '~/components/tree-menu/index.svelte';

	import { featureStore } from '~/components/map4/watershedStore';
	import { watershedStore } from '~/routes/(app)/browse/store';

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

		featureStore.reset();

		for (const child of children) {
			featureStore.add(child.key, child.data);
		}
	};
</script>

<div class="grid grid-cols-[25ch,_1fr]">
	<div class="border-r bg-neutral-100">
		<TreeMenu on:clickedWatershed={onClickedWatershed} />
	</div>
	<div class="bg-slate-100">
		<MapComponent on:ready={onMapReady}>
			<SegmentsProjection />
			<ZoneProjection />
		</MapComponent>
	</div>
</div>
