import { writable, derived } from 'svelte/store';

function createSegmentStore() {
	const { subscribe, set, update } = writable(new Map<string, any>([]));

	return {
		subscribe,
		add: (value: any) =>
			update((n) => {
				console.log(value);
				n.set(value.line.properties.id, value.line);
				n.set(value.start.properties.id, value.start);
				n.set(value.end.properties.id, value.end);
				if (value.zone) {
					n.set(value.zone.properties.id, value.zone);
				}
				return new Map(n);
			}),
		remove: (key: string) =>
			update((n) => {
				for (const mapKey of n.keys()) {
					const part = mapKey.split(':')[0];
					if (part === key) {
						n.delete(mapKey);
					}
				}
				return new Map(n);
			}),
		addSingle: (key: string, value: any) => update((n) => new Map(n.set(key, value))),
		removeSingle: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Map(n);
			}),
		reset: () => set(new Map<string, any>([]))
	};
}

export const segmentStore = createSegmentStore();

export const segmentVisibility = writable(true);
export const zoneVisibility = writable(true);

export const segmentStoreArray = derived(
	[segmentStore, segmentVisibility, zoneVisibility],
	([$sgmentStore, $segmentVisibility, $zoneVisibility]) => {
		const filtered: any[] = [];
		for (const [key, value] of $sgmentStore) {
			if (!$segmentVisibility) {
				if (key.endsWith(':line')) continue;
				if (key.endsWith(':start')) continue;
				if (key.endsWith(':end')) continue;
			}
			if (!$zoneVisibility) {
				if (key.endsWith(':zone')) continue;
			}
			filtered.push(value);
		}
		return filtered;
		// $sgmentStore.entries()
		// return Array.from($sgmentStore.values());
	}
);
