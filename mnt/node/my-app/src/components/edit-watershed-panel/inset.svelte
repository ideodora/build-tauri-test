<script lang="ts">
	import { activeWatershed, watershedStore } from '~/routes/(app)/browse/store';
	import { invoke } from '@tauri-apps/api';

	let { id, name } = Object.assign({}, $activeWatershed);

	const onClickSave = async () => {
		const payload = { id, name };
		await invoke('update_watershed', { payload });

		const watersheds = $watershedStore.map((watershed) => {
			if (watershed.id !== id) return watershed;
			watershed.name = name;
			return watershed;
		});
		$watershedStore = watersheds;
	};
</script>

<div class="wrap pt-2">
	<div class="field mb-4 px-4">
		<label for="item1" class="block">名称</label>
		<input id="item1" type="text" bind:value={name} />
	</div>

	<div class="field mb-4 px-4 opacity-30">
		<label for="item2" class="block">流域県</label>
		<textarea id="item2" rows="2" disabled />
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
	textarea {
		@apply w-full rounded-sm py-1 px-2 text-sm text-gray-700;
	}
</style>
