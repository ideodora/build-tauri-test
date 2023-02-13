<script lang="ts">
	import MapComponent from '~/components/map3/MapComponent.svelte';
	import SegmentsController from '~/components/map3/SegmentsController.svelte';
	import StartingPointsController from '~/components/map3/StartingPointsController.svelte';
	import SearchBlock from '~/components/SearchBlock.svelte';
	import ToolBlock from '~/components/ToolBlock.svelte';

	let startingPointsController: StartingPointsController;
	let segmentsController: SegmentsController;

	const onClickedPin = () => {
		startingPointsController.load();
	};

	const onClickedOffPin = () => {
		startingPointsController.unload();
	};

	const onClickedStartingPoint = (event: any) => {
		segmentsController.loadCurve(event.detail.feature);
	};
</script>

<div class="relative h-full w-full">
	<div class="pointer-events-none absolute inset-0 z-20">
		<div class="pl-2 pt-2">
			<SearchBlock />
		</div>
		<div class="pl-2 pt-2">
			<ToolBlock on:clickedPin={onClickedPin} on:clickedOffPin={onClickedOffPin} />
		</div>
	</div>
	<div class="absolute inset-0 z-10">
		<MapComponent>
			<StartingPointsController
				bind:this={startingPointsController}
				on:clickedPoint={onClickedStartingPoint}
			/>
			<SegmentsController bind:this={segmentsController} />
		</MapComponent>
	</div>
</div>
