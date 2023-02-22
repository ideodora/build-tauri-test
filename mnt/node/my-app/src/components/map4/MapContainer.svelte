<script lang="ts">
	// import LassoControl from '~/components/map3/LassoControl.svelte';
	// import MapComponent from '~/components/map3/MapComponent.svelte';
	// import SegmentsController from '~/components/map3/SegmentsController.svelte';
	import StartingPointsController from '~/components/map4/StartingPointsController.svelte';
	import SearchBlock from '~/components/map4/SearchBlock.svelte';
	import ToolBlock from '~/components/map4/ToolBlock.svelte';
	import MapComponent from '~/components/map4/MapComponent.svelte';
	import SegmentsProjection from '~/components/map4/SegmentsProjection.svelte';
	import LineDrawer from '~/components/map4/LineDrawer.svelte';
	import LassoController from '~/components/map4/LassoController.svelte';
	import ZoneProjection from '~/components/map4/ZoneProjection.svelte';
	import ZoneController from '~/components/map4/ZoneController.svelte';
	import ExportController from '~/components/map4/ExportController.svelte';
	import SegmentController from '~/components/map4/SegmentController.svelte';
	// import { isEditingZone } from '~/components/mapStore';

	// let segmentsController: SegmentsController;

	let zoneController: ZoneController;
	let exportController: ExportController;
	let segmentController: SegmentController;
	let startingPointsController: StartingPointsController;

	const onClickedPin = () => {
		startingPointsController.load();
	};

	const onClickedOffPin = () => {
		startingPointsController.unload();
	};

	const onClickedStartingPoint = (event: any) => {
		segmentController.loadCurve(event.detail.feature);
	};

	const onClickedSearchRiver = (event: any) => {
		segmentController.loadCurveByKey(event.detail.curveKey);
	};

	const onMergedLasso = (event: any) => {
		// segmentsController.onMergedLasso(event);
	};

	const onClickedRemoveSegment = () => {
		// segmentsController.removeSelectingSegments();
	};

	const onClickedSwapStartEnd = () => {
		segmentController.swapStartEndSelecting();
	};

	const onCreateZoneEvent = () => {
		zoneController.buildZone();
	};

	const onExtendZoneEvent = (ev: any) => {
		zoneController.buildZone(ev.detail.extend);
	};

	const onFixedZoneEvent = () => {
		zoneController.fixedZone();
	};

	const onClickedEditZone = () => {
		// $isEditingZone = !$isEditingZone;
	};

	const onClickedExportSegments = () => {
		exportController.exportFeatures();
	};

	const onClickedSave = (ev: any) => {
		exportController.saveActivesAs(ev.detail.fileName);
	};
</script>

<div class="relative h-full">
	<div class="pointer-events-none absolute inset-0 z-20">
		<div class="pl-2 pt-2">
			<SearchBlock on:clickedSearchRiver={onClickedSearchRiver} />
		</div>
		<div class="pl-2 pt-2">
			<ToolBlock
				on:clickedPin={onClickedPin}
				on:clickedOffPin={onClickedOffPin}
				on:clickedRemoveSegment={onClickedRemoveSegment}
				on:clickedSwapStartEnd={onClickedSwapStartEnd}
				on:createZoneEvent={onCreateZoneEvent}
				on:extendZoneEvent={onExtendZoneEvent}
				on:fixedZoneEvent={onFixedZoneEvent}
				on:clickedEditZone={onClickedEditZone}
				on:clickedExportSegments={onClickedExportSegments}
				on:clickedSave={onClickedSave}
			/>
		</div>
	</div>
	<div class="absolute inset-0 z-10">
		<MapComponent autoFocus={false}>
			<StartingPointsController
				bind:this={startingPointsController}
				on:clickedPoint={onClickedStartingPoint}
			/>
			<ExportController bind:this={exportController} />
			<LassoController />
			<LineDrawer />
			<SegmentController bind:this={segmentController} />
			<SegmentsProjection />
			<ZoneController bind:this={zoneController} />
			<ZoneProjection />
		</MapComponent>
	</div>
</div>