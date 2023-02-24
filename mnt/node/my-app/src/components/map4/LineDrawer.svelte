<script lang="ts">
	import { getContext } from 'svelte';
	import { createSegmentFromLayer } from '~/components/map4/feature';
	import { key, L, type MapContext } from '~/components/map4/leaflet';
	import { drawingEnabled } from '~/components/mapStore';
	import { featureStore } from './watershedStore';

	const PM_OPTIONS = {
		snappable: true,
		snapDistance: 20
	};

	const { getMap } = getContext<MapContext>(key);
	const map = getMap();

	map.on('pm:create', ({ layer }) => {
		const polyLine = layer as L.Polyline<GeoJSON.LineString>;

		const segment = createSegmentFromLayer(polyLine);
		featureStore.add(segment.id, segment);

		$drawingEnabled = false;
		map.removeLayer(layer);
	});

	$: {
		if ($drawingEnabled) {
			map.pm.enableDraw('Line', PM_OPTIONS);
		} else {
			map.pm.disableDraw();
		}
	}
</script>
