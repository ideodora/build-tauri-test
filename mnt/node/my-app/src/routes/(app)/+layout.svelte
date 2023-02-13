<script lang="ts">
	import { writable } from 'svelte/store';

	// import { userStore } from '$lib/firebase';
	// import { goto } from '$app/navigation';

	// const user = userStore();

	// $: {
	// 	if ($user === null) {
	// 		goto('/login');
	// 	}
	// }
	let user = writable(true);
</script>

{#if $user === undefined}
	<div class="wrap grid place-items-center">
		<p class="animate-pulse">loading...</p>
	</div>
{:else if $user}
	<div class="wrap">
		<slot />
	</div>
{:else}
	<div class="wrap grid place-items-center">
		<p class="animate-pulse">redirect...</p>
	</div>
{/if}

<style lang="postcss">
	.wrap {
		@apply h-screen w-screen;
	}
</style>
