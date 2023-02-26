import { writable } from 'svelte/store';

export const offscreen = writable<'watershed' | 'segment' | 'zone' | undefined>();
export const editSegment = writable<any>();
export const editZone = writable<any>();

export * from './watershed';
