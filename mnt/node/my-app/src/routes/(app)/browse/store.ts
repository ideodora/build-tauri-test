import { writable, get, derived } from 'svelte/store';

import { activeWatershedId } from '~/components/tree-menu/store';

export const watershedStore = writable<any[]>([]);

export const watershedStore2 = derived(watershedStore, ($watershedStore) => {
	return $watershedStore.map((item) => {
		const segments = [];
		const zones = [];
		for (const child of item.children) {
			if ((child.key as string).endsWith(':segment')) {
				segments.push(child);
			}
			if ((child.key as string).endsWith(':zone')) {
				zones.push(child);
			}
		}
		return { ...item, segments, zones };
	});
});

export const activeWatershed = derived(
	[watershedStore, activeWatershedId],
	([$watershedStore, $activeWatershedId]) => {
		return $watershedStore.find((watershed) => watershed.id === $activeWatershedId);
	}
);
