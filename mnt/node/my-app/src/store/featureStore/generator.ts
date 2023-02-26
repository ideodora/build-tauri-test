import { writable } from 'svelte/store';

import type { Feature } from './types';
export * from './types';

export function createSegmentVisibilityFilter() {
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
					return new Set(n);
				}
				return new Set(n.add(key));
			}),
		reset: () => set(new Set<string>([]))
	};
}

export function createZoneVisibilityFilter() {
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
					return new Set(n);
				}
				return new Set(n.add(key));
			}),
		reset: () => set(new Set<string>([]))
	};
}

export function createFeatureStore() {
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

export function createActiveSegment() {
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

export function createActiveZone() {
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
