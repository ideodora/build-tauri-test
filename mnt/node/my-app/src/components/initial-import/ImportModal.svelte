<script lang="ts">
	import { Dialog, DialogOverlay } from '@rgossiaux/svelte-headlessui';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

	import ProgressCircle from '~/components/initial-import/ProgressCircle.svelte';

	let isOpen = true;
	let isLoaded = false;
	let isImporting = false;
	let unlisten: UnlistenFn;

	let current: string | number = '-';
	let total: string | number = '-';
	let message =
		'Lorem ipsum dolor sit amet, consectetur adipisicing elit. Dolor fugit, nihil unde neque quaerat reiciendis. Et est unde placeat, possimus architecto quod praesentium ad eius saepe ex maiores. Ab, eos!';
	let progress = 0;

	onMount(async () => {
		unlisten = await listen(
			'import-progress',
			(event: { payload: { current: number; total: number; message: string } }) => {
				console.log('event arrived', event);
				current = event.payload.current;
				total = event.payload.total;
				message = event.payload.message;
				progress = Math.round((event.payload.current / event.payload.total) * 100);
			}
		);

		const dataReady = await invoke<boolean>('check_initiation');
		isLoaded = true;
		isOpen = !dataReady;

		return () => {
			unlisten();
		};
	});

	const greet = async () => {
		// Open a selection dialog for image files
		const selected = await open({
			multiple: false,
			filters: [
				{
					name: 'Image',
					extensions: ['png', 'jpeg', 'zip']
				}
			]
		});
		console.dir(selected);
		isImporting = true;
		if (Array.isArray(selected)) {
			// user selected multiple files
		} else if (selected === null) {
			// user cancelled the selection
		} else {
			// user selected a single file
			await invoke('greet', { name: selected });
		}

		// wait data fully loaded
		isOpen = false;
	};

	const cancel = async () => {
		alert('not implemented yet');
	};
</script>

<Dialog open={isOpen} class="fixed inset-0 z-20">
	<DialogOverlay class="absolute inset-0 z-10 bg-black/80" />

	<div class="absolute inset-0 z-20 flex items-center justify-center">
		<div class="relative z-20 rounded-sm bg-white p-8">
			{#if isLoaded && !isImporting}
				<div class="block" in:fade>
					<h1 class="mb-4 text-3xl text-gray-800">First in first.</h1>

					<p class="mb-4">
						Import source.zip for registered points and streams.<br />
						If not, it's no reason to use this App.
					</p>

					<button
						class="rounded-md bg-indigo-500 py-1 px-4 text-white hover:bg-indigo-600"
						on:click={greet}>Import</button
					>
				</div>
			{:else if isLoaded && isImporting}
				<div class="flex flex-wrap gap-4" in:fade>
					<div class="flex flex-col">
						<h1 class="mb-4 text-3xl text-gray-800">Importing...</h1>

						<p class="mb-4 max-w-xs">
							{current} / {total}<br />
							{message}
						</p>

						<div class="mt-auto block">
							<button
								class="rounded-md bg-gray-100 py-1 px-4 text-gray-700 hover:bg-gray-200"
								on:click={cancel}>Cancel</button
							>
						</div>
					</div>
					<div class="h-40 w-40">
						<ProgressCircle {progress} />
					</div>
				</div>
			{:else}
				<div class="animate-pulse">data checking...</div>
			{/if}
		</div>
	</div>
</Dialog>
