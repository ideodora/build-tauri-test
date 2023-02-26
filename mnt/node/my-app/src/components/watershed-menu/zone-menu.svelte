<script lang="ts">
	import Chevron_right from 'svelte-google-materialdesign-icons/Chevron_right.svelte';
	import Expand_more from 'svelte-google-materialdesign-icons/Expand_more.svelte';
	import Visibility from 'svelte-google-materialdesign-icons/Visibility.svelte';
	import Visibility_off from 'svelte-google-materialdesign-icons/Visibility_off.svelte';
	import ZoneItem, {
		type ToggleVisibilityEvent
	} from '~/components/watershed-menu/zone-item.svelte';
	import { zoneVisibilityFilter } from '~/store/featureStore';
	import { store } from '~/store/watershedMenuStore';

	export let parentIndex: number;
	export let index: number;
	export let zones: {
		id: number;
		watershed_id: number;
		key: string;
		name: string;
	}[] = [];

	let visibleAll = true;

	const toggleItemVisibility = (event: ToggleVisibilityEvent) => {
		zoneVisibilityFilter.toggle(event.detail.key);
	};

	const toggleZone = () => {
		visibleAll = !visibleAll;
		for (const zone of zones) {
			if (visibleAll) {
				zoneVisibilityFilter.remove(zone.key);
			} else {
				zoneVisibilityFilter.add(zone.key);
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
			<span class="mr-auto">区画</span>
			<div class="mr-1">
				<button
					type="button"
					class="flex h-[18px] w-[18px] items-center justify-center"
					on:click|preventDefault|stopPropagation={toggleZone}
				>
					{#if visibleAll}
						<Visibility class="text-gray-400" size="18" variation="filled" />
					{:else}
						<Visibility_off class="text-gray-400" size="18" />
					{/if}
				</button>
			</div>
		</summary>
		<ul>
			{#each zones as zone, zoneIndex}
				<ZoneItem index={zoneIndex} data={zone} on:toggleVisibility={toggleItemVisibility} />
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
