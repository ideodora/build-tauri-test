<script lang="ts">
	import { signOut } from '$lib/firebase';

	async function logout() {
		await signOut();
	}
</script>

<button
	class="w-min rounded-md border border-gray-300 bg-white py-1 px-4 text-base font-normal"
	type="button"
	on:click={logout}>logout</button
>
