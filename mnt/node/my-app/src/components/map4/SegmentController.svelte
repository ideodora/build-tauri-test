<script lang="ts">
	import { default as turfClone } from '@turf/clone';
	import { nanoid } from 'nanoid';
	import { getContext } from 'svelte';
	import { createSegment, createSegmentFromCurve } from '~/components/map4/feature';
	import { key, L, type MapContext } from '~/components/map4/leaflet';
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
		const id = `${random}:segment`;

		const line = turfClone(feature.line);
		const newCoords = line.geometry.coordinates.slice();
		newCoords.reverse();

		const newFeature = createSegment(id, newCoords);

		featureStore.add(id, newFeature);
		activeSegment.remove(feature.id);
		featureStore.remove(feature.id);
	};

	const addSegmentsFromCoordinates = (curves: any[]) => {
		return curves.map((curve) => {
			return createSegmentFromCurve(curve);
		});
	};
</script>
