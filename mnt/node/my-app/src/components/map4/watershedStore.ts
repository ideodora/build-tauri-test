import { writable, derived } from 'svelte/store';

export type Feature = SegmentFeature | ZoneFeature;

export type SegmentFeature = {
	kind: 'SegmentFeature';
	id: string;
	line: GeoJSON.Feature<
		GeoJSON.LineString,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentLine';
			weight: number;
			color: string;
		}
	>;
	start: GeoJSON.Feature<
		GeoJSON.Point,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentStart';
			icon: L.DivIcon;
			zIndexOffset: number;
		}
	>;
	end: GeoJSON.Feature<
		GeoJSON.Point,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentEnd';
			icon: L.DivIcon;
			zIndexOffset: number;
		}
	>;
};

export type ZoneFeature = {
	kind: 'ZoneFeature';
	id: string;
	zone: GeoJSON.Feature<
		GeoJSON.Polygon,
		{
			id: string;
			layerId?: number;
			kind: 'Zone';
			weight: number;
			color: string;
			fillOpacity: number;
		}
	>;
};

export const segmentVisibility = writable(true);
export const zoneVisibility = writable(true);

function createFeatureStore() {
	const { subscribe, set, update } = writable(new Map<string, Feature>([]));

	return {
		subscribe,
		add: (key: string, value: Feature) => update((n) => new Map(n.set(key, value))),
		remove: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Map(n);
			}),
		reset: () => set(new Map<string, Feature>([]))
	};
}

export const featureStore = createFeatureStore();

function createActiveSegment() {
	const { subscribe, set, update } = writable(new Set<string>([]));

	return {
		subscribe,
		add: (key: string) => update((n) => new Set(n.add(key))),
		remove: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Set(n);
			}),
		toggle: (key: string) =>
			update((n) => {
				if (n.has(key)) {
					n.delete(key);
				} else {
					n.add(key);
				}
				return new Set(n);
			}),
		set: (key: string) => update((n) => new Set(new Set<string>([key]))),
		reset: () => set(new Set<string>([]))
	};
}

export const activeSegment = createActiveSegment();

function createActiveZone() {
	const { subscribe, set, update } = writable(new Set<string>([]));

	return {
		subscribe,
		add: (key: string) => update((n) => new Set(n.add(key))),
		remove: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Set(n);
			}),
		toggle: (key: string) =>
			update((n) => {
				if (n.has(key)) {
					n.delete(key);
				} else {
					n.add(key);
				}
				return new Set(n);
			}),
		set: (key: string) => update((n) => new Set(new Set<string>([key]))),
		reset: () => set(new Set<string>([]))
	};
}

export const activeZone = createActiveZone();

export const activeFeature = derived(
	[activeSegment, activeZone],
	([$activeSegment, $activeZone]) => {
		return new Set([...$activeSegment, ...$activeZone]);
	}
);

export const featureStoreArray = derived(
	[featureStore, segmentVisibility, zoneVisibility, activeSegment, activeZone],
	([$featureStore, $segmentVisibility, $zoneVisibility, $activeSegment, $activeZone]) => {
		const filtered: Feature[] = [];
		for (const [key, feature] of $featureStore) {
			if ($activeSegment.has(key)) continue;
			if ($activeZone.has(key)) continue;
			if ($segmentVisibility && feature.kind === 'SegmentFeature') {
				filtered.push(feature);
			}
			if ($zoneVisibility && feature.kind === 'ZoneFeature') {
				filtered.push(feature);
			}
		}
		return filtered;
	}
);

export const activeFeatureStoreArray = derived(
	[featureStore, activeSegment, activeZone],
	([$featureStore, $activeSegment, $activeZone]) => {
		const filtered: Feature[] = [];
		for (const feature of $featureStore.values()) {
			if ($activeSegment.has(feature.id)) {
				filtered.push(feature);
			}
			if ($activeZone.has(feature.id)) {
				filtered.push(feature);
			}
		}
		return filtered;
	}
);

export function isZoneFeature(feature: Feature): feature is ZoneFeature {
	return feature.kind === 'ZoneFeature';
}

export function isSegmentFeature(feature: Feature): feature is SegmentFeature {
	return feature.kind === 'SegmentFeature';
}
