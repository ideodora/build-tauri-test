import { derived, writable } from 'svelte/store';

// lasso relates
// cutting mode switch
export const isCutting = writable<boolean>(false);
// lasso mode switch
export const lassoEnabled = writable<boolean>(false);
// lasso continue
export const lassoContinue = writable<boolean>(false);
