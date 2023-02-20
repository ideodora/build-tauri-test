<script lang="ts">
	import { store } from './store';
	import Chevron_right from 'svelte-google-materialdesign-icons/Chevron_right.svelte';
	import Expand_more from 'svelte-google-materialdesign-icons/Expand_more.svelte';
	import Visibility from 'svelte-google-materialdesign-icons/Visibility.svelte';
	import Visibility_off from 'svelte-google-materialdesign-icons/Visibility_off.svelte';
	import Insert_drive_file from 'svelte-google-materialdesign-icons/Insert_drive_file.svelte';
	import Edit from 'svelte-google-materialdesign-icons/Edit.svelte';
	import File_download from 'svelte-google-materialdesign-icons/File_download.svelte';
	import Content_copy from 'svelte-google-materialdesign-icons/Content_copy.svelte';
	import { createEventDispatcher } from 'svelte';
	import { watershedStore } from '~/routes/(app)/browse/store';
	import { segmentVisibility, zoneVisibility } from '~/components/map3/watershedStore';

	const dispatch = createEventDispatcher();

	const onClickWatershed = (index: number) => {
		store.toggle(index);
		dispatch('clickedWatershed', { index });
	};

	const toggleSegment = () => {
		$segmentVisibility = !$segmentVisibility;
	};

	const toggleZone = () => {
		$zoneVisibility = !$zoneVisibility;
	};
</script>

<ul class="tree text-xs font-thin text-gray-800">
	{#each $watershedStore as item, index0}
		<li>
			<details open={$store.get(`${index0}`)}>
				<summary
					class="bg-white"
					on:click={() => onClickWatershed(index0)}
					class:outline={index0 === 1}
					class:outline-1={index0 === 1}
					class:outline-stone-400={index0 === 1}
					class:-outline-offset-1={index0 === 1}
				>
					{#if $store.get(`${index0}`)}
						<Expand_more size="20" />
					{:else}
						<Chevron_right size="20" />
					{/if}
					<span class="mr-auto">{item.name}</span>
					<div class="mr-1 flex gap-x-1">
						{#if index0 === 1}
							<div class="flex h-[18px] w-[18px] items-center justify-center rotate-x-180">
								<Content_copy size="16" />
							</div>
							<File_download size="18" />
							<div class="relative h-[18px] w-[18px]">
								<Insert_drive_file size="18" />
								<div class="absolute bottom-0 right-0">
									<Edit size="12" stroke="white" stroke-width="5" />
								</div>
								<div class="absolute bottom-0 right-0">
									<Edit size="12" />
								</div>
							</div>
						{/if}
					</div>
				</summary>
				{#if item.children.length > 0}
					<ul>
						{#each [true, true] as itemL2, index1}
							<li>
								<details open={$store.get(`${index0}-${index1}`)}>
									<summary class="pl-2" on:click={() => store.toggle(index0, index1)}>
										{#if $store.get(`${index0}-${index1}`)}
											<Expand_more size="20" />
										{:else}
											<Chevron_right size="20" />
										{/if}
										<span class="mr-auto">{index1 === 0 ? '流路' : '区画'}</span>
										<div class="mr-1">
											{#if index1 === 0}
												<button
													type="button"
													class="flex h-[18px] w-[18px] items-center justify-center"
													on:click|preventDefault|stopPropagation={toggleSegment}
												>
													{#if $segmentVisibility}
														<Visibility class="text-gray-400" size="18" variation="filled" />
													{:else}
														<Visibility_off class="text-gray-400" size="18" />
													{/if}
												</button>
											{:else}
												<button
													type="button"
													class="flex h-[18px] w-[18px] items-center justify-center"
													on:click|preventDefault|stopPropagation={toggleZone}
												>
													{#if $zoneVisibility}
														<Visibility class="text-gray-400" size="18" variation="filled" />
													{:else}
														<Visibility_off class="text-gray-400" size="18" />
													{/if}
												</button>
											{/if}
										</div>
									</summary>
									{#if item.children.length > 0}
										<ul>
											{#each item.children as itemL3, index2}
												<li class="flex py-1 pl-10">
													<span class="mr-auto">{itemL3.key}</span>
													<div class="mr-2">
														{#if index2 === 0}
															<Visibility class="text-gray-400" size="18" variation="filled" />
														{:else}
															<Visibility_off class="text-gray-400" size="18" />
														{/if}
													</div>
												</li>
											{/each}
										</ul>
									{/if}
								</details>
							</li>
						{/each}
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
