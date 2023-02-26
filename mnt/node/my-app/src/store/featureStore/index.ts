import { derived } from 'svelte/store';

import type { Feature } from './types';
export * from './types';

import {
	createActiveSegment,
	createActiveZone,
	createFeatureStore,
	createSegmentVisibilityFilter,
	createZoneVisibilityFilter
} from './generator';

export const featureStore = createFeatureStore();

export const segmentVisibilityFilter = createSegmentVisibilityFilter();
export const zoneVisibilityFilter = createZoneVisibilityFilter();

export const activeSegment = createActiveSegment();
export const activeZone = createActiveZone();
export const activeFeature = derived(
	[activeSegment, activeZone],
	([$activeSegment, $activeZone]) => {
		return new Set([...$activeSegment, ...$activeZone]);
	}
);

export const featureStoreArray = derived(featureStore, ($featureStore) => {
	return [...$featureStore.values()];
});

export const displayFeatureStoreArray = derived(
	[featureStore, activeFeature, segmentVisibilityFilter, zoneVisibilityFilter],
	([$featureStore, $activeFeature, $segmentVisibilityFilter, $zoneVisibilityFilter]) => {
		const filtered: Feature[] = [];
		for (const [key, feature] of $featureStore) {
			if ($activeFeature.has(key)) continue;
			if ($segmentVisibilityFilter.has(key)) continue;
			if ($zoneVisibilityFilter.has(key)) continue;
			filtered.push(feature);
		}
		return filtered;
	}
);

export const activeFeatureStoreArray = derived(
	[featureStore, activeFeature],
	([$featureStore, $activeFeature]) => {
		const filtered: Feature[] = [];
		for (const feature of $featureStore.values()) {
			if ($activeFeature.has(feature.id)) {
				filtered.push(feature);
			}
		}
		return filtered;
	}
);
