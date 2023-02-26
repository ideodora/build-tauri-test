import { derived } from 'svelte/store';

import { createActiveWatershedId, createWatershedMapStore } from './generator';

export const activeWatershedId = createActiveWatershedId();

export const watershedMapStore = createWatershedMapStore();

export const watershedStore = derived(watershedMapStore, ($watershedMapStore) => {
	const array = [...$watershedMapStore.values()];
	return array.map((item) => {
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
