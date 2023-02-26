<script context="module" lang="ts">
	export type ToggleVisibilityEvent = CustomEvent<{ key: string }>;
</script>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Edit from 'svelte-google-materialdesign-icons/Edit.svelte';
	import Insert_drive_file from 'svelte-google-materialdesign-icons/Insert_drive_file.svelte';
	import Visibility from 'svelte-google-materialdesign-icons/Visibility.svelte';
	import Visibility_off from 'svelte-google-materialdesign-icons/Visibility_off.svelte';
	import { zoneVisibilityFilter } from '~/store/featureStore';
	import { editZone, offscreen } from '~/store/browseStore';

	const dispatcher = createEventDispatcher<{ toggleVisibility: { key: string } }>();

	export let index: number = 0;
	export let data: {
		id: number;
		watershed_id: number;
		key: string;
		name: string;
	};

	let computedName: string = '';

	$: {
		if (data.name) {
			computedName = data.name;
		} else {
			computedName = `ゾーンー${index + 1}`;
		}
	}

	const toggleVisibility = () => {
		dispatcher('toggleVisibility', { key: data.key });
	};

	const onClickEdit = () => {
		$offscreen = 'zone';
		$editZone = undefined;
		setTimeout(() => {
			$editZone = data;
		}, 100);
	};
</script>

<li class="flex py-1 pl-10">
	<span class="mr-auto">{computedName}</span>
	<div class="mr-2 flex gap-x-1">
		<button
			type="button"
			class="flex h-[18px] w-[18px] items-center justify-center"
			on:click|preventDefault|stopPropagation={onClickEdit}
		>
			<div class="relative h-[18px] w-[18px] text-gray-400">
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
			on:click|preventDefault|stopPropagation={toggleVisibility}
		>
			{#if $zoneVisibilityFilter.has(data.key)}
				<Visibility_off class="text-gray-400" size="18" />
			{:else}
				<Visibility class="text-gray-400" size="18" variation="filled" />
			{/if}
		</button>
	</div>
</li>
