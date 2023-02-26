import type L from 'leaflet';
import { derived, writable } from 'svelte/store';

export { isCutting, lassoEnabled, lassoContinue } from './lasso';
export { drawingEnabled } from './drawing';
export { isComposingZone, isEditingZone } from './zone';

// starting points source
export const rawStartingPoints = writable<any[]>([]);

// sync map center
export const syncCenter = writable<L.LatLng>();

// map bounds
export const segmentsBounds = writable<L.LatLngBounds | undefined>();
export const zonesBounds = writable<L.LatLngBounds | undefined>();
export const focusBounds = derived(
	[segmentsBounds, zonesBounds],
	([$segmentsBounds, $zonesBounds]) => {
		if (!$segmentsBounds && !$zonesBounds) {
			return undefined;
		}

		if ($segmentsBounds && $zonesBounds) {
			// extend is side effect so clone first by pad0 = same bounds
			const clone = $segmentsBounds.pad(0);
			return clone.extend($zonesBounds);
		}

		return $segmentsBounds ? $segmentsBounds : $zonesBounds;
	}
);
