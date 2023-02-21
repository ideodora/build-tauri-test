<script lang="ts">
	import L from 'leaflet';
	import 'leaflet-lasso';
	import { lassoEnabled, lassoContinue } from '~/components/mapStore';
	import { FINISHED_EVENT, type LassoHandlerFinishedEvent } from 'leaflet-lasso';
	import { getContext } from 'svelte';
	import { endIcon, startIcon, key, type MapContext } from '~/components/map4/leaflet';
	import {
		activeSegment,
		featureStore,
		type SegmentFeature
	} from '~/components/map4/watershedStore';
	import { nanoid } from 'nanoid';
	import { lineString, point as turfPoint } from '@turf/helpers';

	const { getMap } = getContext<MapContext>(key);
	const map = getMap();

	let lasso = L.lasso(map);

	// TODO:
	// something to do with MODE

	map.on(FINISHED_EVENT, (ev: unknown) => {
		lassoEnabled.set(false);

		const event = ev as LassoHandlerFinishedEvent;
		mergeSegment(event);

		if ($lassoContinue) {
			lassoEnabled.set(true);
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

	$: {
		console.log($lassoEnabled);
		$lassoEnabled ? lasso.enable() : lasso.disable();
	}
</script>
