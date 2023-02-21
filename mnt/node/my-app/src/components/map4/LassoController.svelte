<script lang="ts">
	import L from 'leaflet';
	import 'leaflet-lasso';
	import { lassoEnabled, lassoContinue, isCutting } from '~/components/mapStore';
	import { FINISHED_EVENT, type LassoHandlerFinishedEvent } from 'leaflet-lasso';
	import { getContext } from 'svelte';
	import { endIcon, startIcon, key, type MapContext } from '~/components/map4/leaflet';
	import {
		activeFeatureStoreArray,
		activeSegment,
		featureStore,
		isSegmentFeature,
		type SegmentFeature
	} from '~/components/map4/watershedStore';
	import { nanoid } from 'nanoid';
	import { lineString, point as turfPoint } from '@turf/helpers';
	import lineToPolygon from '@turf/line-to-polygon';
	import lineSplit from '@turf/line-split';
	import booleanWithin from '@turf/boolean-within';
	import lineSliceAlong from '@turf/line-slice-along';

	const { getMap } = getContext<MapContext>(key);
	const map = getMap();

	let lasso = L.lasso(map);

	// TODO:
	// something to do with MODE

	map.on(FINISHED_EVENT, (ev: unknown) => {
		const event = ev as LassoHandlerFinishedEvent;

		if ($lassoEnabled) {
			mergeSegment(event);
			$lassoEnabled = false;

			if ($lassoContinue) {
				lassoEnabled.set(true);
			}
		}

		if ($isCutting) {
			cutSegment(event);
			$isCutting = false;
		}
	});

	const mergeSegment = (event: LassoHandlerFinishedEvent) => {
		const features = event.layers.map((layer: any) => layer.feature);

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
		const formerSegmentId = ends[0].properties.id.split(':').slice(0, -1).join(':');
		const latterSegmentId = starts[0].properties.id.split(':').slice(0, -1).join(':');

		// get segment from store
		const former = $featureStore.get(formerSegmentId)! as SegmentFeature;
		const latter = $featureStore.get(latterSegmentId)! as SegmentFeature;

		// extract lines
		const formerLine = former.line;
		const latterLine = latter.line;

		// unite coodinates
		const formerCoords = formerLine.geometry.coordinates;
		const latterCoords = latterLine.geometry.coordinates;
		const merged = [...formerCoords, ...latterCoords];

		const random = nanoid();
		const id = `${random}:segment`;

		const line = createLineFeature(id, merged);
		const start = createStartFeature(id, merged);
		const end = createEndFeature(id, merged);

		const segment = {
			kind: 'SegmentFeature' as const,
			id,
			line,
			start,
			end
		} satisfies SegmentFeature;

		featureStore.add(id, segment);
		featureStore.remove(formerSegmentId);
		featureStore.remove(latterSegmentId);

		activeSegment.reset();
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

	const cutSegment = (event: LassoHandlerFinishedEvent) => {
		const coords = L.GeoJSON.latLngsToCoords(event.latLngs);

		// // confirmation
		// if (!confirm('部分除去を実行します。よろしいですか？')) {
		//   // this.cutSegmentsDone();
		//   return;
		// }

		const feature = $activeFeatureStoreArray[0];
		if (!feature || !isSegmentFeature(feature)) return;

		const poly = lineToPolygon(lineString(coords));
		const split = lineSplit(feature.line, poly);

		// remove original segment
		featureStore.remove(feature.id);
		activeSegment.reset();

		// this is separated line by polygon
		// remove inside polygon part, add outerside as new segment
		if (split.features.length > 0) {
			split.features.forEach((splitedPart, i) => {
				if (!booleanWithin(lineSliceAlong(splitedPart, 1, 1, { units: 'meters' }), poly)) {
					const random = nanoid();
					const id = `${random}:segment`;

					const apartSegment = splitedPart.geometry.coordinates;

					const line = createLineFeature(id, apartSegment);
					const start = createStartFeature(id, apartSegment);
					const end = createEndFeature(id, apartSegment);

					const segment = {
						kind: 'SegmentFeature' as const,
						id,
						line,
						start,
						end
					} satisfies SegmentFeature;

					featureStore.add(id, segment);
				}
			});
		}
	};

	$: {
		console.log($lassoEnabled);
		$lassoEnabled ? lasso.enable() : lasso.disable();
	}

	$: {
		console.log($isCutting);
		$isCutting ? lasso.enable() : lasso.disable();
	}
</script>
