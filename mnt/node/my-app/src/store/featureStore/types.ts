export type Feature = SegmentFeature | ZoneFeature;

export type SegmentFeature = {
	kind: 'SegmentFeature';
	id: string;
	line: GeoJSON.Feature<
		GeoJSON.LineString,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentLine';
			weight: number;
			color: string;
		}
	>;
	start: GeoJSON.Feature<
		GeoJSON.Point,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentStart';
			icon: L.DivIcon;
			zIndexOffset: number;
		}
	>;
	end: GeoJSON.Feature<
		GeoJSON.Point,
		{
			id: string;
			layerId?: number;
			kind: 'SegmentEnd';
			icon: L.DivIcon;
			zIndexOffset: number;
		}
	>;
};

export type ZoneFeature = {
	kind: 'ZoneFeature';
	id: string;
	zone: GeoJSON.Feature<
		GeoJSON.Polygon,
		{
			id: string;
			layerId?: number;
			kind: 'Zone';
			weight: number;
			color: string;
			fillOpacity: number;
		}
	>;
};

export function isZoneFeature(feature: Feature): feature is ZoneFeature {
	return feature.kind === 'ZoneFeature';
}

export function isSegmentFeature(feature: Feature): feature is SegmentFeature {
	return feature.kind === 'SegmentFeature';
}
