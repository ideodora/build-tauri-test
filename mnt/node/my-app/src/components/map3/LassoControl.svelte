<script lang="ts">
	import L from 'leaflet';
	import 'leaflet-lasso';
	import { lassoEnabled, lassoContinue } from '~/components/mapStore';
	import { FINISHED_EVENT } from 'leaflet-lasso';
	import { getContext } from 'svelte';
	import { key, type MapContext } from '~/components/map3/leaflet';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	const { getMap } = getContext<MapContext>(key);
	const map = getMap();

	let lasso = L.lasso(map);

	// TODO:
	// something to do with MODE

	map.on(FINISHED_EVENT, (ev: unknown) => {
		console.log('lasso.finished in LassoControl');
		lassoEnabled.set(false);

		dispatch('mergedLasso', ev);

		if ($lassoContinue) {
			lassoEnabled.set(true);
		}

		// stop ongoing selection
		// this.deactivateSegment();

		// when is cut MODE
		//
		// cutSegments(customEvent);
	});

	$: {
		console.log($lassoEnabled);
		$lassoEnabled ? lasso.enable() : lasso.disable();
	}
</script>
