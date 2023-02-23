<script lang="ts">
	import { store } from './store';
	import { segmentVisibility, segmentVisibilityFilter } from '~/components/map4/watershedStore';
	import Visibility from 'svelte-google-materialdesign-icons/Visibility.svelte';
	import Visibility_off from 'svelte-google-materialdesign-icons/Visibility_off.svelte';
	import Expand_more from 'svelte-google-materialdesign-icons/Expand_more.svelte';
	import Chevron_right from 'svelte-google-materialdesign-icons/Chevron_right.svelte';
	import SegmentItem, {
		type ToggleVisibilityEvent
	} from '~/components/watershed-menu/segment-item.svelte';

	export let parentIndex: number;
	export let index: number;
	export let segments: {
		id: number;
		watershed_id: number;
		key: string;
		name: string;
	}[] = [];

	const toggleItemVisibility = (event: ToggleVisibilityEvent) => {
		segmentVisibilityFilter.toggle(event.detail.key);
	};

	const toggleZone = () => {
		$segmentVisibility = !$segmentVisibility;
		for (const segment of segments) {
			if ($segmentVisibility) {
				segmentVisibilityFilter.remove(segment.key);
			} else {
				segmentVisibilityFilter.add(segment.key);
			}
		}
	};
</script>

<li>
	<details open={$store.get(`${parentIndex}-${index}`)}>
		<summary class="pl-2" on:click={() => store.toggle(parentIndex, index)}>
			{#if $store.get(`${parentIndex}-${index}`)}
				<Expand_more size="20" />
			{:else}
				<Chevron_right size="20" />
			{/if}
			<span class="mr-auto">流路</span>
			<div class="mr-1">
				<button
					type="button"
					class="flex h-[18px] w-[18px] items-center justify-center"
					on:click|preventDefault|stopPropagation={toggleZone}
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
			{#each segments as segment, segmentIndex}
				<SegmentItem
					index={segmentIndex}
					data={segment}
					on:toggleVisibility={toggleItemVisibility}
				/>
			{/each}
		</ul>
	</details>
</li>

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
