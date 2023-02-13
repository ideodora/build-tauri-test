<script lang="ts">
	import L from 'leaflet';
	import 'leaflet-lasso';
	import { lassoEnabled, lassoContinue } from '~/components/mapStore';
	import { FINISHED_EVENT } from 'leaflet-lasso';
	import SegmentMerge from './SegmentMerger.svelte';

	export let map: L.Map;

	let lasso = L.lasso(map);

	// TODO:
	// something to do with MODE

	map.on(FINISHED_EVENT, (ev: unknown) => {
		console.log('lasso.finished in LassoControl');
		lassoEnabled.set(false);

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

<SegmentMerge {map} />
