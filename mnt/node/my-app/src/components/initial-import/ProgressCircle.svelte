<script lang="ts">
	export let progress = 0;
	let dashOffset = 100;
	$: dashOffset = 100 - progress;
</script>

<div class="relative h-full w-full">
	<svg
		class="circle-container h-full w-full"
		viewBox="2 -2 28 36"
		xmlns="http://www.w3.org/2000/svg"
	>
		<circle class="circle-container__background" r="16" cx="16" cy="16" />
		<circle
			class="circle-container__progress"
			r="16"
			cx="16"
			cy="16"
			style="stroke-dashoffset: {dashOffset}"
		/>
	</svg>
	<div class="absolute inset-0 flex items-center justify-center">
		<div>
			<span class="text-3xl text-gray-800">{progress}</span><span class="text-gray-600">%</span>
		</div>
	</div>
</div>

<style lang="postcss">
	:root {
		--dot-diameter: 380px;
		--circle-border-width: 2px;
		--default-color: gainsboro;
		--completion-color: theme(colors.indigo.500);
	}

	.circle-container {
		/* width: var(--dot-diameter);
		height: var(--dot-diameter); */
		transform: rotate(-90deg);
	}

	.circle-container__background {
		fill: none;
		stroke: var(--default-color);
		stroke-width: var(--circle-border-width);
	}

	.circle-container__progress {
		fill: none;
		stroke-linecap: round;
		stroke: var(--completion-color);
		stroke-dasharray: 100 100;
		stroke-width: var(--circle-border-width);

		transition: stroke-dashoffset 1s ease-in-out;
		will-change: transform;
	}
</style>
