<script lang="ts">
	// import MapContainer from '~/components/map3/MapContainer.svelte';
	import MapContainer from '~/components/map4/MapContainer.svelte';
	import ImportModal from '~/components/initial-import/ImportModal.svelte';
	import { beforeNavigate, goto } from '$app/navigation';
	import type { BeforeNavigate } from '@sveltejs/kit';
	import { confirm } from '@tauri-apps/api/dialog';

	let left = false;

	beforeNavigate(async ({ to, cancel }) => {
		if (!left) {
			cancel();

			const confirmed = await confirm('移動確認', {
				title: '編集途中のものは削除されますがよろしいですか？',
				type: 'warning'
			});

			if (confirmed) {
				left = true;
				if (to) {
					goto(to.url);
				}
				return;
			}
		}
	});
</script>

<ImportModal />
<MapContainer />
