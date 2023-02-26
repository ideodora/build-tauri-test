<script lang="ts">
	import { default as turfClone } from '@turf/clone';
	import { nanoid } from 'nanoid';
	import { getContext } from 'svelte';
	import { createSegmentFromCurve } from '~/components/map4/feature';
	import { endIcon, key, L, startIcon, type MapContext } from '~/components/map4/leaflet';
	import {
		activeFeatureStoreArray,
		activeSegment,
		featureStore,
		isSegmentFeature,
		type SegmentFeature
	} from '~/store/featureStore';
	import { loadCurve as _loadCurve, loadCurveByKey as _loadCurveByKey } from './loadCurve';

	const { getMap } = getContext<MapContext>(key);

	const map = getMap();

	export const swapStartEndSelecting = () => {
		for (const feature of $activeFeatureStoreArray) {
			if (!isSegmentFeature(feature)) continue;
			swapStartEnd(feature);
		}
	};

	export const loadCurve = async (feature: any) => {
		const curves = await _loadCurve(feature);
		console.log('curves', curves);
		const segments = addSegmentsFromCoordinates(curves);
		for (const segment of segments) {
			featureStore.add(segment.id, segment);
		}
	};

	export const loadCurveByKey = async (curveKey: any) => {
		const curves = await _loadCurveByKey(curveKey);
		const segments = addSegmentsFromCoordinates(curves);
		for (const segment of segments) {
			featureStore.add(segment.id, segment);
		}

		const lines = segments.map((segment) => segment.line);
		const geoJson = L.geoJSON(lines);
		const bounds = geoJson.getBounds();
		map.fitBounds(bounds);
	};

	const swapStartEnd = (feature: SegmentFeature) => {
		const random = nanoid();
		const segmentId = `${random}:segment`;

		const line = turfClone(feature.line);
		const newCoords = line.geometry.coordinates.slice();
		newCoords.reverse();
		line.geometry.coordinates = newCoords;
		line.properties.id = `${segmentId}:line`;
		line.properties.layerId = undefined;

		const start = turfClone(feature.end);
		start.properties.id = `${segmentId}:start`;
		start.properties.kind = 'SegmentStart';
		start.properties.layerId = undefined;
		start.properties.icon = startIcon();

		const end = turfClone(feature.start);
		end.properties.id = `${segmentId}:end`;
		end.properties.kind = 'SegmentEnd';
		end.properties.layerId = undefined;
		end.properties.icon = endIcon();

		const newFeature = {
			kind: 'SegmentFeature',
			id: segmentId,
			line,
			start,
			end
		} satisfies SegmentFeature;

		debugger;

		featureStore.add(segmentId, newFeature);
		activeSegment.remove(feature.id);
		featureStore.remove(feature.id);
	};

	const addSegmentsFromCoordinates = (curves: any[]) => {
		return curves.map((curve) => {
			return createSegmentFromCurve(curve);
		});
	};
</script>
