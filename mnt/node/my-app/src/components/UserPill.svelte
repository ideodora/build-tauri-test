<script lang="ts">
	import { userStore } from '$lib/firebase';

	const user = userStore();
</script>

{#if $user}
	<div>
		<h2>{$user.displayName}</h2>
	</div>
{/if}
