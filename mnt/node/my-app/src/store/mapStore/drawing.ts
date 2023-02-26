import { writable } from 'svelte/store';

export const drawingEnabled = writable<boolean>(false);
