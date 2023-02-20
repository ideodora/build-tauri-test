import { writable, get } from 'svelte/store';

function createStore() {
	const store = writable(new Map<string, boolean>([]));
	const { subscribe, set, update } = store;

	return {
		subscribe,
		toggle: (...indexes: number[]) =>
			update((n) => {
				const key = indexes.join('-');
				const current = n.get(key) ?? false;
				n.set(key, !current);
				return new Map(n);
			}),
		getByKey: (...indexes: number[]) => get(store).get(indexes.join('-'))
	};
}

export const store = createStore();
