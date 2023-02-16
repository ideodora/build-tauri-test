<script lang="ts">
	import Timeline from 'svelte-google-materialdesign-icons/Timeline.svelte';
	import Commit from 'svelte-google-materialdesign-icons/Commit.svelte';
	import Pin_drop from 'svelte-google-materialdesign-icons/Pin_drop.svelte';
	import Location_off from 'svelte-google-materialdesign-icons/Location_off.svelte';
	import Select_all from 'svelte-google-materialdesign-icons/Select_all.svelte';
	import Save_as from 'svelte-google-materialdesign-icons/Save_as.svelte';
	import Repeat from 'svelte-google-materialdesign-icons/Repeat.svelte';
	import Layers_clear from 'svelte-google-materialdesign-icons/Layers_clear.svelte';
	import Content_cut from 'svelte-google-materialdesign-icons/Content_cut.svelte';
	import Swap_vert from 'svelte-google-materialdesign-icons/Swap_vert.svelte';

	import { getBounds, addStartingPoint, removeStartingPoints } from '~/components/Map.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import {
		sps,
		lassoEnabled,
		lassoContinue,
		asgstr,
		drawingEnabled,
		isComposingZone
	} from '~/components/mapStore';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	$: showSps = $sps.length > 0;

	async function onPin() {
		dispatch('clickedPin', {});
		return;

		const bounds = getBounds();
		console.log(bounds);

		const { lat: swLat, lng: swLng } = bounds.getSouthWest();
		const { lat: neLat, lng: neLng } = bounds.getNorthEast();

		const params = {
			min: {
				lat: swLat,
				lng: swLng
			},
			max: {
				lat: neLat,
				lng: neLng
			}
		};

		const startingPoints = await invoke<any>('search_points', { bounds: params });

		console.log(startingPoints.points);
		sps.set(startingPoints.points);

		// removeStartingPoints();
		// for (const point of startingPoints.points) {
		// 	addStartingPoint(point);
		// }
	}

	function offPin() {
		dispatch('clickedOffPin', {});
		return;
		sps.set([]);
	}

	function offSelect() {
		asgstr.reset();
	}

	function addZone() {
		dispatch('createZoneEvent', {});
		// zsgstr.set(100);
	}

	function removeSegment() {
		dispatch('clickedRemoveSegment', {});
	}

	function swapStartEnd() {
		dispatch('clickedSwapStartEnd', {});
	}
</script>

<div class="pointer-events-auto col-span-2 flex w-max items-start gap-x-2">
	<button
		class="btn"
		type="button"
		class:!bg-red-200={$drawingEnabled}
		on:click={() => {
			$drawingEnabled = !$drawingEnabled;
		}}
	>
		<Timeline tabindex="-1" class="outline-none" />
	</button>
	<div class="flex flex-col">
		<button
			class="btn mb-1"
			type="button"
			class:!bg-indigo-200={$lassoEnabled}
			on:click={() => {
				$lassoEnabled = !$lassoEnabled;
			}}
		>
			<Commit tabindex="-1" class="outline-none" />
		</button>
		{#if $lassoEnabled}
			<button
				class="btn"
				type="button"
				class:!bg-red-200={$lassoContinue}
				on:click={() => {
					$lassoContinue = !$lassoContinue;
				}}
			>
				<Repeat tabindex="-1" class="outline-none" />
			</button>
		{/if}
	</div>
	<div class="flex flex-col">
		<button class="btn mb-1" type="button" on:click={onPin}>
			<Pin_drop tabindex="-1" class="outline-none" variation="filled" />
		</button>
		{#if showSps}
			<button class="btn" type="button" on:click={offPin}>
				<Location_off tabindex="-1" class="outline-none" variation="filled" />
			</button>
		{/if}
	</div>
	{#if $asgstr.size === 1}
		<button class="btn" type="button" on:click={addZone} class:!bg-indigo-200={$isComposingZone}>
			<Select_all class="outline-none" tabindex="-1" />
		</button>
		<button class="btn" type="button" on:click={swapStartEnd}>
			<Swap_vert tabindex="-1" />
		</button>
		<button class="btn" type="button" on:click={offSelect}>
			<Content_cut class="outline-none" tabindex="-1" />
		</button>
	{/if}
	{#if $asgstr.size > 0}
		<button class="btn" type="button" on:click={offSelect}>
			<Save_as class="outline-none" tabindex="-1" />
		</button>
		<button class="btn !bg-red-200" type="button" on:click={removeSegment}>
			<Layers_clear class="outline-none" tabindex="-1" />
		</button>
		<button class="btn" type="button" on:click={offSelect}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				x="0px"
				y="0px"
				width="24"
				height="24"
				viewBox="0,0,256,256"
				style="fill:#000000;"
			>
				<g transform="translate(19.69231,19.69231) scale(0.84615,0.84615)"
					><g
						fill="#000000"
						fill-rule="nonzero"
						stroke="none"
						stroke-width="1"
						stroke-linecap="butt"
						stroke-linejoin="miter"
						stroke-miterlimit="10"
						stroke-dasharray=""
						stroke-dashoffset="0"
						font-family="none"
						font-weight="none"
						font-size="none"
						text-anchor="none"
						style="mix-blend-mode: normal"
						><g transform="scale(9.84615,9.84615)"
							><path
								d="M7,2.01563l13.16016,12.32813l-5.375,0.48438l-0.97656,0.08984l0.40625,0.89453l3.26172,7.14453l-2.40625,1.05859l-3.11328,-7.22266l-0.39062,-0.91016l-0.72266,0.67969l-3.81641,3.58203l-0.02734,-18.12891M7,0.01563c-0.26953,0 -0.53906,0.05078 -0.79687,0.16406c-0.73047,0.31641 -1.20312,1.03906 -1.20312,1.83594l0.02734,18.12891c0,0.79688 0.47656,1.51563 1.20313,1.83203c0.25781,0.10938 0.52734,0.16406 0.79297,0.16406c0.5,0 0.99219,-0.1875 1.37109,-0.53906l2.46094,-2.3125l2.37891,5.51563c0.21094,0.49219 0.60938,0.875 1.10547,1.07031c0.23438,0.09375 0.48047,0.14063 0.73047,0.14063c0.27344,0 0.54688,-0.05859 0.80469,-0.17187l2.40625,-1.05859c0.48828,-0.21484 0.875,-0.61719 1.0625,-1.11719c0.19141,-0.5 0.17578,-1.05469 -0.04687,-1.54297l-2.5,-5.46875l3.54297,-0.32422c0.78516,-0.07031 1.45703,-0.59375 1.71094,-1.33984c0.25781,-0.74609 0.05078,-1.57031 -0.52344,-2.10937l-13.16016,-12.32812c-0.37891,-0.35547 -0.87109,-0.53906 -1.36719,-0.53906z"
							/></g
						></g
					></g
				>
			</svg>
		</button>
	{/if}
</div>

<style lang="postcss">
	.btn {
		@apply aspect-square w-min rounded-md border border-gray-400 bg-white p-1 shadow-md shadow-indigo-600/30 transition hover:-translate-y-px hover:bg-gray-200 hover:shadow-lg hover:shadow-indigo-600/20;
	}
</style>
