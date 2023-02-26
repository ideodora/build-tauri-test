<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { editZone, offscreen, watershedMapStore, watershedStore } from '~/store/browseStore';

	let { id, watershed_id, name } = Object.assign({}, $editZone);

	const onClickSave = async () => {
		const payload = { id, name };
		await invoke('update_watershed_item', { payload });

		const watershed = $watershedMapStore.get(watershed_id);
		const children = watershed.children.map((child: any) => {
			if (child.id !== id) return child;
			child.name = name;
			return child;
		});
		watershed.children = children;
		watershedMapStore.set(watershed_id, watershed);

		$offscreen = undefined;
		$editZone = undefined;
	};
</script>

<div class="wrap pt-2">
	<div class="field mb-4 px-4">
		<label for="item1" class="block">名称</label>
		<input
			id="item1"
			class="placeholder:text-gray-400"
			type="text"
			bind:value={name}
			placeholder="未設定"
		/>
	</div>

	<div class="field mb-4 px-4 text-right">
		<button
			on:click={onClickSave}
			class="rounded-md bg-indigo-500 py-1 px-4 text-white hover:bg-indigo-600 active:bg-indigo-700"
			>保存</button
		>
	</div>
</div>

<style lang="postcss">
	label {
		@apply text-xs leading-loose text-gray-600;
	}
	input {
		@apply w-full rounded-sm py-1 px-2 text-sm text-gray-700;
	}
</style>
