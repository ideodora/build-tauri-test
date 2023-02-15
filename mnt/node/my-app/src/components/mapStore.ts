import type L from 'leaflet';
import { writable } from 'svelte/store';

export const sMap = writable<L.Map>();
export const sps = writable<any[]>([]);
export const segments = writable<any[]>([]);
export const lassoEnabled = writable<boolean>(false);
export const lassoContinue = writable<boolean>(false);
export const segmentsStore = writable<Map<string, any>>(new Map([]));

export const drawingEnabled = writable<boolean>(false);

function createSegmentStore() {
	const { subscribe, set, update } = writable(new Map<string, any>([]));

	return {
		subscribe,
		add: (key: string, value: any) => update((n) => new Map(n.set(key, value))),
		remove: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Map(n);
			})
	};
}

export const sgstr = createSegmentStore();

function createActiveSegmentStore() {
	const { subscribe, set, update } = writable(new Map<string, any>([]));

	return {
		subscribe,
		reset: () => set(new Map([])),
		set: (key: string, value: any) => set(new Map([[key, value]])),
		add: (key: string, value: any) =>
			update((n) => {
				if (n.get(key)) {
					n.delete(key);
					return new Map(n);
				}
				return new Map(n.set(key, value));
			}),
		remove: (key: string) =>
			update((n) => {
				n.delete(key);
				return new Map(n);
			})
	};
}

export const asgstr = createActiveSegmentStore();

function createZoneSegmentStore() {
	const { subscribe, set } = writable(0);

	return {
		subscribe,
		set: (num: number) => set(num)
	};
}

export const zsgstr = createZoneSegmentStore();
