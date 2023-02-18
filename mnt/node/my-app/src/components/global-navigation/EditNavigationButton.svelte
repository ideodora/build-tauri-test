<script lang="ts">
	import Shape_line from 'svelte-google-materialdesign-icons/Shape_line.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	import { open } from '@tauri-apps/api/dialog';

	let name = 'misawa';
	let greetMsg = 'ryota';

	// class="grid w-full grow-0 grid-flow-col grid-rows-[min-content] items-center justify-start border-t border-gray-200 py-2 px-3 shadow-md"
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
		if (Array.isArray(selected)) {
			// user selected multiple files
		} else if (selected === null) {
			// user cancelled the selection
		} else {
			// user selected a single file
		}

		greetMsg = await invoke('greet', { name: selected });
	};

	// async function greet() {
	// 	greetMsg = await invoke('greet', { name });
	// 	console.log(greetMsg);
	// }
</script>

<a href="/" class="w-min rounded-md bg-white py-1 px-2 hover:bg-gray-200">
	<Shape_line tabindex="-1" />
</a>
