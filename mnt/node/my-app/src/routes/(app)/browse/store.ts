import { writable, get, derived } from 'svelte/store';

import { activeWatershedId } from '~/components/tree-menu/store';

export const watershedStore = writable<any[]>([]);

export const activeWatershed = derived(
	[watershedStore, activeWatershedId],
	([$watershedStore, $activeWatershedId]) => {
		return $watershedStore.find((watershed) => watershed.id === $activeWatershedId);
	}
);
