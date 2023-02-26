<script lang="ts">
	import Clear from 'svelte-google-materialdesign-icons/Clear.svelte';
	import Edit from 'svelte-google-materialdesign-icons/Edit.svelte';
	import Inset from '~/components/edit-watershed-panel/inset.svelte';
	import { activeWatershed, offscreen } from '~/store/browseStore';
</script>

<div class="flex h-full flex-col">
	<div
		class="flex items-center justify-between border-b border-t border-gray-300 bg-white py-2 px-4"
	>
		<div class="mr-2 block">
			<Edit class="text-gray-600" />
		</div>
		<div class="mr-auto block">
			<div class="text-xs leading-tight text-gray-500">編集</div>
			<div class="text-base leading-none text-gray-800">流域属性</div>
		</div>
		<button on:click={() => ($offscreen = undefined)}>
			<Clear class="text-gray-600" tabindex="-1" />
		</button>
	</div>
	<div class="flex-grow overflow-auto">
		{#if $activeWatershed}
			<Inset />
		{/if}
	</div>
</div>
