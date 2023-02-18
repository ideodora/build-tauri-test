<script lang="ts">
	import { writable } from 'svelte/store';
	import Navigation from '~/components/global-navigation/Navigation.svelte';

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

<div class="wrap">
	{#if $user === undefined}
		<div class="wrap grid place-items-center">
			<p class="animate-pulse">loading...</p>
		</div>
	{:else if $user}
		<div class="grid h-full w-full grid-rows-[1fr,_min-content]">
			<slot />
			<Navigation />
		</div>
	{:else}
		<div class="wrap grid place-items-center">
			<p class="animate-pulse">redirect...</p>
		</div>
	{/if}
</div>

<style lang="postcss">
	.wrap {
		@apply h-screen w-screen;
	}
</style>
