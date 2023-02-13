<script lang="ts">
	import L from 'leaflet';
	import union from '@turf/union';
	import { polygon } from '@turf/helpers';
	import { sgstr, segments as storeSegments } from '~/components/mapStore';
	import { FINISHED_EVENT, type LassoHandlerFinishedEvent } from 'leaflet-lasso';
	import { nanoid } from 'nanoid';

	export let map: L.Map;

	type FeaturePropertyKind =
		| 'SegmentLine'
		| 'SegmentStart'
		| 'SegmentEnd'
		| 'SegmentZone'
		| 'StartingPoint';

	type FeatureProperty = {
		id: string;
		kind: FeaturePropertyKind;
	};

	type FeaturePropertySegmentZone = FeatureProperty & {
		kind: 'SegmentZone';
		fillOpacity: number;
		weight: number;
		color: string;
	};

	// TODO:
	// something to do with MODE
	map.on(FINISHED_EVENT, (ev: unknown) => {
		const event = ev as LassoHandlerFinishedEvent;
		console.log('lasso.finished in SegmentMerger', event);

		const lassoShape = L.GeoJSON.latLngsToCoords(event.latLngs);
		const targetFeatures = event.layers.map((layer: any) => layer.feature);
		const customEvent = { lassoShape, targetFeatures };

		console.dir(customEvent);

		// when is merge MODE
		mergeSegments(customEvent.targetFeatures);
		// if continue do it again else end
		// if (this.continueMerging) {
		//   setTimeout(() => {
		//     this.startMergingSegments();
		//   }, 10);
		// } else {
		//   this.cancelMergingSegments();
		// }
	});
	const mergeSegments = (
		features: GeoJSON.Feature<
			GeoJSON.LineString | GeoJSON.Point | GeoJSON.Polygon,
			FeatureProperty
		>[]
	) => {
		// // confirmation
		// if (!this.continueMerging && !confirm('連結しますか?')) {
		// 	return;
		// }

		// extract starts and ends
		const starts = features.filter((feature) => feature.properties.kind === 'SegmentStart');
		const ends = features.filter((feature) => feature.properties.kind === 'SegmentEnd');

		// if not single pair error
		if (starts.length !== 1 || ends.length !== 1) {
			alert('no single conjunction');
			return;
		}

		// id extraction
		const formerId = ends[0].properties.id.split(':')[0];
		const latterId = starts[0].properties.id.split(':')[0];

		// get segment from store
		const former = $sgstr.get(formerId);
		const latter = $sgstr.get(latterId);

		// extract lines
		const formerLine = former.line;
		const latterLine = latter.line;

		// unite coodinates
		const formerCorrds = formerLine!.geometry.coordinates as GeoJSON.Position[];
		const latterCorrds = latterLine!.geometry.coordinates as GeoJSON.Position[];
		const merged = formerCorrds.concat(latterCorrds);

		// add to store with new id
		const id = nanoid();
		const segmentId = `mergedLine-${id}`;
		// // this.addSegment(segmentId, merged);
		const newSegment = { curveId: segmentId, segments: merged };
		const mapper = new Map($storeSegments.map((item) => [item.curveId, item]));
		mapper.set(segmentId, newSegment);
		mapper.delete(formerId);
		mapper.delete(latterId);
		storeSegments.set([...mapper.values()]);

		// // zone merging
		// const formerZone = former.zone;
		// const latterZone = latter.zone;
		// const zones = [formerZone, latterZone].filter((v) => v) as GeoJSON.Feature<
		// 	GeoJSON.Polygon,
		// 	FeaturePropertySegmentZone
		// >[];

		// TODO: merge zone
		// // if only 2 separated zones
		// if (zones.length === 2) {
		// 	const newZone = union(zones[0], zones[1]);
		// 	// only if single polygon not multipolygon
		// 	if (newZone?.geometry.type === 'Polygon') {
		// 		debugger;
		// 		const newZoneId = `${segmentId}:zone`;
		// 		const newFeature = polygon(newZone.geometry.coordinates, {
		// 			id: newZoneId,
		// 			kind: 'SegmentZone' as const,
		// 			color: '#00FF00',
		// 			weight: 1,
		// 			fillOpacity: 0.3
		// 		}) as GeoJSON.Feature<GeoJSON.Polygon, FeaturePropertySegmentZone>;
		// 		this.segments[segmentId].features.push(newFeature);
		// 		this.segments[segmentId].zone = newFeature;
		// 		this.lib.addDataToGeoJson(segmentId, newFeature);
		// 	}
		// }

		// // need outside function?
		// $segmentsStore.set(segmentId, merged);
		// $segmentsStore.delete(formerId);
		// $segmentsStore.delete(latterId);
		// // clone
		// $segmentsStore = new Map($segmentsStore);
	};
</script>
