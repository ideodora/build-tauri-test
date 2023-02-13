<script lang="ts">
	import { goto } from '$app/navigation';
	import { userStore, signOut } from '$lib/firebase';
	const user = userStore();

	import { onMount } from 'svelte';

	console.log('app page 1111');

	onMount(() => {
		console.log('app page 2222');
	});

	async function logout() {
		await signOut();
	}

	async function goaway() {
		await goto('/login');
	}
</script>

<h1 class="text-3xl font-bold">this is dashboard</h1>

<button class="rounded-md bg-white py-1 px-4" type="button" on:click={logout}>logout</button>

{#if $user === undefined}
	<h2>loading...</h2>
{:else if $user}
	<h2>{$user.uid}</h2>
{:else}
	<h2>goout</h2>
{/if}

<p><a href="/">home</a></p>
<p><a href="/login">login</a></p>
