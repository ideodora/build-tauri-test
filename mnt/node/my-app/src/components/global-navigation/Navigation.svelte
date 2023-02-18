<script lang="ts">
	import BrowseNavigationButton from '~/components/global-navigation/BrowseNavigationButton.svelte';
	import EditNavigationButton from '~/components/global-navigation/EditNavigationButton.svelte';
	import Logo from '~/components/global-navigation/Logo.svelte';
	import LogoutButton from '~/components/global-navigation/LogoutButton.svelte';
	import UserPill from '~/components/global-navigation/UserPill.svelte';

	import { open } from '@tauri-apps/api/dialog';

	// class="grid w-full grow-0 grid-flow-col grid-rows-[min-content] items-center justify-start border-t border-gray-200 py-2 px-3 shadow-md"
	const loadFile = async () => {
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
		if (Array.isArray(selected)) {
			// user selected multiple files
		} else if (selected === null) {
			// user cancelled the selection
		} else {
			// user selected a single file
		}
	};
</script>

<div
	class=" flex w-full grow-0 items-center justify-start border-t border-gray-200 py-2 px-6 shadow-md"
>
	<Logo />
	<div class="ml-8 flex gap-x-2">
		<EditNavigationButton />
		<BrowseNavigationButton />
		<button type="button" on:click={loadFile}>btn</button>
	</div>
	<div class="ml-auto flex items-center gap-x-2">
		<UserPill />
		<LogoutButton />
	</div>
</div>
