<script lang="ts">
	import { getContext } from 'svelte';
	import { endIcon, key, L, startIcon, type MapContext } from '~/components/map4/leaflet';
	import { drawingEnabled } from '~/components/mapStore';
	import { nanoid } from 'nanoid';
	import { lineString, point as turfPoint } from '@turf/helpers';
	import { featureStore, type SegmentFeature } from './watershedStore';

	const PM_OPTIONS = {
		snappable: true,
		snapDistance: 20
	};

	const { getMap } = getContext<MapContext>(key);
	const map = getMap();

	map.on('pm:create', ({ layer }) => {
		const polyLine = layer as L.Polyline<GeoJSON.LineString>;

		createSegment(polyLine);

		$drawingEnabled = false;
		map.removeLayer(layer);
	});

	const createSegment = (polyLine: L.Polyline<GeoJSON.LineString>) => {
		const feature = polyLine.toGeoJSON();
		const random = nanoid();
		const id = `${random}:segment`;

		const line = createLineFeature(id, feature.geometry.coordinates);
		const start = createStartFeature(id, feature.geometry.coordinates);
		const end = createEndFeature(id, feature.geometry.coordinates);

		const segment = {
			kind: 'SegmentFeature' as const,
			id,
			line,
			start,
			end
		} satisfies SegmentFeature;

		featureStore.add(id, segment);
	};

	const createLineFeature = (id: string, coordinates: GeoJSON.Position[]) => {
		return lineString(coordinates, {
			id: `${id}:line`,
			layerId: undefined,
			kind: 'SegmentLine' as const,
			weight: 5,
			color: '#0000FF'
		});
	};

	const createStartFeature = (id: string, coordinates: GeoJSON.Position[]) => {
		return turfPoint(coordinates[0], {
			id: `${id}:start`,
			layerId: undefined,
			kind: 'SegmentStart' as const,
			icon: startIcon(),
			zIndexOffset: 1000
		});
	};

	const createEndFeature = (id: string, coordinates: GeoJSON.Position[]) => {
		return turfPoint(coordinates[coordinates.length - 1], {
			id: `${id}:end`,
			layerId: undefined,
			kind: 'SegmentEnd' as const,
			icon: endIcon(),
			zIndexOffset: 2000
		});
	};

	$: {
		if ($drawingEnabled) {
			map.pm.enableDraw('Line', PM_OPTIONS);
		} else {
			map.pm.disableDraw();
		}
	}
</script>
