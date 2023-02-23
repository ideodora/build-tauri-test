<script lang="ts">
	import { activeWatershedId, store } from './store';
	import Chevron_right from 'svelte-google-materialdesign-icons/Chevron_right.svelte';
	import Expand_more from 'svelte-google-materialdesign-icons/Expand_more.svelte';
	import Insert_drive_file from 'svelte-google-materialdesign-icons/Insert_drive_file.svelte';
	import Edit from 'svelte-google-materialdesign-icons/Edit.svelte';
	import File_download from 'svelte-google-materialdesign-icons/File_download.svelte';
	import Content_copy from 'svelte-google-materialdesign-icons/Content_copy.svelte';
	import Clear from 'svelte-google-materialdesign-icons/Clear.svelte';
	import { createEventDispatcher } from 'svelte';
	import {
		featureStoreArray,
		isSegmentFeature,
		isZoneFeature
	} from '~/components/map4/watershedStore';
	import { save as dialogSave } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import tokml from 'tokml';

	export let index: number;
	export let watershed: {
		id: number;
		name: string;
	};

	const dispatch = createEventDispatcher();

	const onClickHeading = (index: number) => {
		store.toggle(index);
	};

	const deleteWatershed = () => {
		dispatch('deleteWatershed');
	};

	const editWatershed = () => {
		dispatch('editWatershed');
	};

	const exportVisible = async () => {
		const path = await dialogSave({
			defaultPath: 'exports.zip',
			filters: [
				{
					name: 'Zip',
					extensions: ['zip']
				}
			]
		});
		if (path) {
			const mapper = new Map<string, string>([]);

			for (const feature of $featureStoreArray) {
				if (isSegmentFeature(feature)) {
					const line = feature.line;
					const lineKml = tokml(line);
					console.log(line.properties.id);
					mapper.set(convertIdToKmlFileName(line.properties.id), lineKml);
				}
				if (isZoneFeature(feature)) {
					const zone = feature.zone;
					const zoneKml = tokml(zone);
					console.log(zone.properties.id);
					mapper.set(convertIdToKmlFileName(zone.properties.id), zoneKml);
				}
			}

			const body = {
				path,
				data: [...mapper.entries()]
			};

			await invoke('save_export', { body });
		}
	};

	const convertIdToKmlFileName = (id: string) => {
		const replaced = id.replaceAll(':', '_');
		return `${replaced}.kml`;
	};
</script>

<summary
	class="bg-white"
	on:click={() => onClickHeading(index)}
	class:outline={$activeWatershedId === watershed.id}
	class:outline-1={$activeWatershedId === watershed.id}
	class:outline-stone-400={$activeWatershedId === watershed.id}
	class:-outline-offset-1={$activeWatershedId === watershed.id}
>
	{#if $store.get(`${index}`)}
		<Expand_more size="20" />
	{:else}
		<Chevron_right size="20" />
	{/if}

	<span class="mr-auto">{watershed.name}</span>

	{#if $activeWatershedId === watershed.id}
		<div class="mr-1 flex gap-x-1">
			<div class="flex h-[18px] w-[18px] items-center justify-center rotate-x-180">
				<Content_copy size="16" />
			</div>
			<button
				type="button"
				class="flex h-[18px] w-[18px] items-center justify-center"
				on:click|preventDefault|stopPropagation={exportVisible}
			>
				<File_download size="18" />
			</button>

			<button
				type="button"
				class="flex h-[18px] w-[18px] items-center justify-center"
				on:click|preventDefault|stopPropagation={editWatershed}
			>
				<div class="relative h-[18px] w-[18px]">
					<Insert_drive_file size="18" />
					<div class="absolute bottom-0 right-0">
						<Edit size="12" stroke="white" stroke-width="5" />
					</div>
					<div class="absolute bottom-0 right-0">
						<Edit size="12" />
					</div>
				</div>
			</button>

			<button
				type="button"
				class="flex h-[18px] w-[18px] items-center justify-center"
				on:click|preventDefault|stopPropagation={deleteWatershed}
			>
				<Clear size="18" />
			</button>
		</div>
	{/if}
</summary>

<style lang="postcss">
	summary {
		display: flex;
		@apply cursor-pointer select-none items-center py-1 pr-1;
	}

	summary::marker,
	summary::-webkit-details-marker {
		display: none;
	}
</style>
