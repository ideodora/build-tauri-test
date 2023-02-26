<script lang="ts">
	import { beforeNavigate, goto } from '$app/navigation';
	import { confirm } from '@tauri-apps/api/dialog';
	import ImportModal from '~/components/initial-import/ImportModal.svelte';
	import MapContainer from '~/components/map4/MapContainer.svelte';
	import { activeSegment, activeZone } from '~/store/featureStore';

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
				$activeSegment.clear();
				$activeZone.clear();
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
