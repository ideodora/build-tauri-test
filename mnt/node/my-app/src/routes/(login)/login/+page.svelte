<script lang="ts">
	import { signInWithEmailAndPassword$ } from '$lib/firebase';
	import { goto } from '$app/navigation';

	let email: string;
	let password: string;

	async function login() {
		try {
			const user = await signInWithEmailAndPassword$(email, password);
			if (user) {
				goto('/');
			}
		} catch (error) {
			console.log('error signin in', error);
		}
	}
</script>

<div class="grid h-full w-full items-center justify-center">
	<div class="block">
		<p class="mb-6 text-xl">Login...</p>

		<div class="mb-8 grid grid-cols-[40px,_12rem] items-center gap-2">
			<label for="email" class="text-right"> email: </label>
			<input
				class="rounded-md py-1"
				bind:value={email}
				type="email"
				placeholder="Enter your email"
				name="email"
			/>
			<label for="password" class="text-right"> pass: </label>
			<input
				class="rounded-md py-1"
				bind:value={password}
				type="password"
				placeholder="Enter password"
				name="password"
			/>
			<button
				class="col-start-2 mt-2 rounded-md border bg-indigo-500 py-1 px-4 text-white hover:bg-indigo-600"
				on:click={login}>Login</button
			>
		</div>
	</div>
</div>
