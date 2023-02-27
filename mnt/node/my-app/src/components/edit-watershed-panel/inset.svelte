<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { activeWatershed, offscreen, watershedMapStore } from '~/store/browseStore';
	import { prefs } from '~/store/prefs';
	import MultiSelect from 'svelte-multiselect';

	const ui_libs = prefs;

	let { id, name, prefectures } = Object.assign({}, $activeWatershed);

	let selected: { label: string; value: number }[] = prefectures.map((item: any) => {
		return { value: item.id, label: item.long_name };
	});

	const onClickSave = async () => {
		const pref_ids = selected.map((item) => item.value);
		const payload = { id, name, pref_ids };
		await invoke('update_watershed', { payload });

		const watershed = $watershedMapStore.get(id);
		watershed.name = name;
		watershed.prefectures = selected.map((item: any) => ({
			id: item.value,
			long_name: item.label
		}));
		watershedMapStore.set(id, watershed);

		$offscreen = undefined;
	};
</script>

<div class="wrap pt-2">
	<div class="field mb-4 px-4">
		<label for="item1" class="block">名称</label>
		<input id="item1" type="text" bind:value={name} />
	</div>

	<div class="field mb-4 px-4">
		<label for="item2" class="block">流域県</label>
		<MultiSelect
			bind:selected
			options={ui_libs}
			outerDivClass={'!bg-white !border-gray-500 !rounded-sm'}
			liSelectedClass={'!bg-blue-500 !text-white !text-sm'}
			liOptionClass={'!text-sm !text-gray-700'}
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
