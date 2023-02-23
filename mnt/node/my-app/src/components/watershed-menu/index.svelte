<script lang="ts">
	import { watershedStore2 } from '~/routes/(app)/browse/store';
	import WatershedItem from '~/components/watershed-menu/watershed-item.svelte';
</script>

<ul class="watershed-menu text-xs font-thin text-gray-800">
	{#each $watershedStore2 as watershed, index}
		<WatershedItem {index} {watershed} on:deleteWatershed on:editWatershed />
	{/each}
</ul>
