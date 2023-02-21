<script lang="ts">
	import { default as turfClone } from '@turf/clone';
	import { nanoid } from 'nanoid';
	import { endIcon, startIcon, L } from '~/components/map4/leaflet';
	import {
		activeFeatureStoreArray,
		activeSegment,
		featureStore,
		isSegmentFeature,
		type SegmentFeature
	} from '~/components/map4/watershedStore';
	import { loadCurve as _loadCurve, loadCurveByKey as _loadCurveByKey } from './loadCurve';
	import { lineString, point as turfPoint } from '@turf/helpers';

	export const swapStartEndSelecting = () => {
		for (const feature of $activeFeatureStoreArray) {
			if (!isSegmentFeature(feature)) continue;
			swapStartEnd(feature);
		}
	};

	export const loadCurve = async (feature: any) => {
		const curves = await _loadCurve(feature);
		console.log('curves', curves);
		addSegmentsFromCoordinates(curves);
	};

	export const loadCurveByKey = async (curveKey: any) => {
		const curves = await _loadCurveByKey(curveKey);
		console.log('curves', curves);
		addSegmentsFromCoordinates(curves);
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
		for (const curve of curves) {
			addSegmentFromCoordinates(curve);
		}
	};

	const addSegmentFromCoordinates = (curve: any) => {
		const curveId = curve.curveId;
		const id = `${curveId}:segment`;

		const line = createLineFeature(id, curve.segments);
		const start = createStartFeature(id, curve.segments);
		const end = createEndFeature(id, curve.segments);

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
</script>
