<script lang="ts">
	import { activeWatershedId, store } from './store';
	import Chevron_right from 'svelte-google-materialdesign-icons/Chevron_right.svelte';
	import Expand_more from 'svelte-google-materialdesign-icons/Expand_more.svelte';
	import Visibility from 'svelte-google-materialdesign-icons/Visibility.svelte';
	import Visibility_off from 'svelte-google-materialdesign-icons/Visibility_off.svelte';
	import Insert_drive_file from 'svelte-google-materialdesign-icons/Insert_drive_file.svelte';
	import Edit from 'svelte-google-materialdesign-icons/Edit.svelte';
	import File_download from 'svelte-google-materialdesign-icons/File_download.svelte';
	import Content_copy from 'svelte-google-materialdesign-icons/Content_copy.svelte';
	import Clear from 'svelte-google-materialdesign-icons/Clear.svelte';
	import { createEventDispatcher } from 'svelte';
	import { watershedStore, watershedStore2 } from '~/routes/(app)/browse/store';
	import {
		featureStoreArray,
		isSegmentFeature,
		isZoneFeature,
		segmentVisibility,
		segmentVisibilityFilter,
		zoneVisibility,
		zoneVisibilityFilter
	} from '~/components/map4/watershedStore';
	import { save as dialogSave } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import tokml from 'tokml';

	const dispatch = createEventDispatcher();

	const onClickWatershedKeydown = (e: KeyboardEvent, id: number) => {
		if (e.key === 'Enter') {
			onClickWatershed(id);
		}
	};
	const onClickWatershed = (id: number) => {
		$activeWatershedId = id;
	};

	const onClickHeading = (index: number) => {
		store.toggle(index);
	};

	const deleteWatershed = () => {
		dispatch('deleteWatershed');
	};

	const toggleSegment = (segments: any[]) => {
		$segmentVisibility = !$segmentVisibility;
		for (const segment of segments) {
			if ($segmentVisibility) {
				segmentVisibilityFilter.remove(segment.key);
			} else {
				segmentVisibilityFilter.add(segment.key);
			}
		}
	};
	const toggleSegmentVisibility = (key: string) => {
		segmentVisibilityFilter.toggle(key);
	};

	const toggleZone = (segments: any[]) => {
		$zoneVisibility = !$zoneVisibility;
		for (const segment of segments) {
			if ($zoneVisibility) {
				zoneVisibilityFilter.remove(segment.key);
			} else {
				zoneVisibilityFilter.add(segment.key);
			}
		}
	};
	const toggleZoneVisibility = (key: string) => {
		zoneVisibilityFilter.toggle(key);
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

<ul class="tree text-xs font-thin text-gray-800">
	{#each $watershedStore2 as item, index0}
		<li
			on:keydown={(e) => onClickWatershedKeydown(e, item.id)}
			on:click={() => onClickWatershed(item.id)}
		>
			<details open={$store.get(`${index0}`)}>
				<summary
					class="bg-white"
					on:click={() => onClickHeading(index0)}
					class:outline={$activeWatershedId === item.id}
					class:outline-1={$activeWatershedId === item.id}
					class:outline-stone-400={$activeWatershedId === item.id}
					class:-outline-offset-1={$activeWatershedId === item.id}
				>
					{#if $store.get(`${index0}`)}
						<Expand_more size="20" />
					{:else}
						<Chevron_right size="20" />
					{/if}

					<span class="mr-auto">{item.name}</span>

					{#if $activeWatershedId === item.id}
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
							<div class="relative h-[18px] w-[18px]">
								<Insert_drive_file size="18" />
								<div class="absolute bottom-0 right-0">
									<Edit size="12" stroke="white" stroke-width="5" />
								</div>
								<div class="absolute bottom-0 right-0">
									<Edit size="12" />
								</div>
							</div>
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
				{#if item.children.length > 0}
					<ul>
						{#if item.segments.length > 0}
							<li>
								<details open={$store.get(`${index0}-0`)}>
									<summary class="pl-2" on:click={() => store.toggle(index0, 0)}>
										{#if $store.get(`${index0}-0`)}
											<Expand_more size="20" />
										{:else}
											<Chevron_right size="20" />
										{/if}
										<span class="mr-auto">流路</span>
										<div class="mr-1">
											<button
												type="button"
												class="flex h-[18px] w-[18px] items-center justify-center"
												on:click|preventDefault|stopPropagation={() => toggleSegment(item.segments)}
											>
												{#if $segmentVisibility}
													<Visibility class="text-gray-400" size="18" variation="filled" />
												{:else}
													<Visibility_off class="text-gray-400" size="18" />
												{/if}
											</button>
										</div>
									</summary>
									<ul>
										{#each item.segments as itemL3, index2}
											<li class="flex py-1 pl-10">
												<span class="mr-auto">流路ー{index2 + 1}</span>
												<div class="mr-2">
													<button
														type="button"
														class="flex h-[18px] w-[18px] items-center justify-center"
														on:click|preventDefault|stopPropagation={() =>
															toggleSegmentVisibility(itemL3.key)}
													>
														{#if $segmentVisibilityFilter.has(itemL3.key)}
															<Visibility_off class="text-gray-400" size="18" />
														{:else}
															<Visibility class="text-gray-400" size="18" variation="filled" />
														{/if}
													</button>
												</div>
											</li>
										{/each}
									</ul>
								</details>
							</li>
						{/if}
						{#if item.zones.length > 0}
							<li>
								<details open={$store.get(`${index0}-1`)}>
									<summary class="pl-2" on:click={() => store.toggle(index0, 1)}>
										{#if $store.get(`${index0}-1`)}
											<Expand_more size="20" />
										{:else}
											<Chevron_right size="20" />
										{/if}
										<span class="mr-auto">区画</span>
										<div class="mr-1">
											<button
												type="button"
												class="flex h-[18px] w-[18px] items-center justify-center"
												on:click|preventDefault|stopPropagation={() => toggleZone(item.zones)}
											>
												{#if $zoneVisibility}
													<Visibility class="text-gray-400" size="18" variation="filled" />
												{:else}
													<Visibility_off class="text-gray-400" size="18" />
												{/if}
											</button>
										</div>
									</summary>
									<ul>
										{#each item.zones as itemL3, index2}
											<li class="flex py-1 pl-10">
												<span class="mr-auto">ゾーンー{index2 + 1}</span>
												<div class="mr-2">
													<button
														type="button"
														class="flex h-[18px] w-[18px] items-center justify-center"
														on:click|preventDefault|stopPropagation={() =>
															toggleZoneVisibility(itemL3.key)}
													>
														{#if $zoneVisibilityFilter.has(itemL3.key)}
															<Visibility_off class="text-gray-400" size="18" />
														{:else}
															<Visibility class="text-gray-400" size="18" variation="filled" />
														{/if}
													</button>
												</div>
											</li>
										{/each}
									</ul>
								</details>
							</li>
						{/if}
					</ul>
				{/if}
			</details>
		</li>
	{/each}
</ul>

<style lang="postcss">
	.tree summary {
		display: flex;
		@apply cursor-pointer select-none items-center py-1 pr-1;
	}

	.tree summary::marker,
	.tree summary::-webkit-details-marker {
		display: none;
	}

	.tree summary:focus {
		/* outline: none; */
	}

	.tree summary:focus-visible {
		/* outline: 1px dotted #000; */
	}
</style>
