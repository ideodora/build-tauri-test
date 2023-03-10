import { lineString, point as turfPoint } from '@turf/helpers';
import { nanoid } from 'nanoid';
import { endIcon, L, startIcon } from '~/components/map4/leaflet';
import type { SegmentFeature } from '~/store/featureStore';

export const createSegmentFromCurve = (curve: any) => {
	const curveId = curve.curveId;
	const id = `${curveId}:segment`;
	const coordinates = curve.segments;

	return createSegment(id, coordinates);
};

export const createSegmentFromLayer = (polyLine: L.Polyline<GeoJSON.LineString>) => {
	const feature = polyLine.toGeoJSON();
	const random = nanoid();
	const id = `${random}:segment`;
	const coordinates = feature.geometry.coordinates;

	return createSegment(id, coordinates);
};

export const createSegment = (id: string, coordinates: GeoJSON.Position[]) => {
	const line = createLineFeature(id, coordinates);
	const start = createStartFeature(id, coordinates);
	const end = createEndFeature(id, coordinates);

	const segment = {
		kind: 'SegmentFeature' as const,
		id,
		line,
		start,
		end
	} satisfies SegmentFeature;

	return segment;
};

export const createLineFeature = (id: string, coordinates: GeoJSON.Position[]) => {
	return lineString(coordinates, {
		id: `${id}:line`,
		layerId: undefined,
		kind: 'SegmentLine' as const,
		weight: 5,
		color: '#0000FF'
	});
};

export const createStartFeature = (id: string, coordinates: GeoJSON.Position[]) => {
	return turfPoint(coordinates[0], {
		id: `${id}:start`,
		layerId: undefined,
		kind: 'SegmentStart' as const,
		icon: startIcon(),
		zIndexOffset: 1000
	});
};

export const createEndFeature = (id: string, coordinates: GeoJSON.Position[]) => {
	return turfPoint(coordinates[coordinates.length - 1], {
		id: `${id}:end`,
		layerId: undefined,
		kind: 'SegmentEnd' as const,
		icon: endIcon(),
		zIndexOffset: 2000
	});
};
