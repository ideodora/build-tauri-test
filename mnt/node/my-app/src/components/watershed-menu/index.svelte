<script lang="ts">
	import WatershedItem from '~/components/watershed-menu/watershed-item.svelte';
	import { watershedStore } from '~/store/browseStore';
</script>

<ul class="watershed-menu text-xs font-thin text-gray-800">
	{#each $watershedStore as watershed, index}
		<WatershedItem {index} {watershed} on:deleteWatershed on:editWatershed />
	{/each}
</ul>
