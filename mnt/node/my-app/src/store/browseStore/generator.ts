import { writable } from 'svelte/store';

export function createActiveWatershedId() {
	const { subscribe, set, update } = writable<number>(-1);

	return {
		subscribe,
		reset: () => set(-1),
		set,
		update
	};
}

export function createWatershedMapStore() {
	const { subscribe, set, update } = writable<Map<number, any>>(new Map([]));

	return {
		subscribe,
		fromArray: (array: any[]) => {
			const map = new Map<number, any>([]);
			for (const item of array) {
				map.set(item.id, item);
			}
			return set(map);
		},
		remove: (key: number) =>
			update((n) => {
				n.delete(key);
				return new Map(n);
			}),
		reset: () => set(new Map()),
		set: (key: number, value: any) =>
			update((n) => {
				n.set(key, value);
				return new Map(n);
			})
	};
}
