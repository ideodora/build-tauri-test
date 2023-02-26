import { writable } from 'svelte/store';

export const isComposingZone = writable<boolean>(false);
export const isEditingZone = writable<boolean>(false);
