<script lang="ts">
	import Search from 'svelte-google-materialdesign-icons/Search.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { fly } from 'svelte/transition';
	import { segments as storeSegments } from '~/components/mapStore';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	type Candidate = {
		river_name: string;
		river_code: string;
		prefecture_name: string;
	};

	$: visible = false;
	let query: string = '植田';
	let candidates: Candidate[] = [];

	const search = async () => {
		visible = true;
		const ret: { candidates: Candidate[] } = await invoke('candidates', { query });
		console.log(ret);
		candidates = ret.candidates;
	};

	// TODO: loadRiver;

	const onLoadRiverKeydown = async (event: KeyboardEvent, candidate: any) => {
		if (event.key === 'Enter') {
			await loadRiver(candidate);
		}
	};
	const loadRiver = async (candidate: any) => {
		const curveKey = { name: candidate.river_name, river_code: candidate.river_code };
		dispatch('clickedSearchRiver', { curveKey });
		visible = false;
		return;

		// 	const { pid, prefCode } = event.detail.feature.properties;
		// 	const curveId = { pid, pref_code: prefCode };
		const result = await invoke<any>('get_curves', { curveKey });
		console.log(result);
		result.curves.forEach((curve: any) => {
			const splits: string[] = curve.segments.split('\\\\r\\\\n');
			const segments = splits.map((split) => {
				const splitStrs = split.split(' ');
				return [parseFloat(splitStrs[1]), parseFloat(splitStrs[0])];
			});
			const segment = { curveId: curve.id, segments };
			const mapper = new Map($storeSegments.map((item) => [item.curveId, item]));
			mapper.set(curve.id, segment);
			storeSegments.set([...mapper.values()]);
		});
	};
</script>

<div
	class="pointer-events-auto grid w-min grid-cols-[20rem,_max-content] gap-x-px shadow-md shadow-indigo-600/30 transition hover:shadow-lg hover:shadow-indigo-600/20 hover:-translate-y-px"
>
	<input
		bind:value={query}
		type="text"
		class="rounded-l-sm border-0 text-sm outline outline-1 outline-gray-400"
	/>
	<button
		on:click={search}
		type="button"
		class="rounded-r-sm bg-indigo-500 px-2 text-white outline outline-1 outline-gray-400 hover:bg-indigo-600 focus:bg-indigo-600"
	>
		<Search tabindex="-1" />
	</button>
</div>
<div
	class="pointer-events-auto mt-1 w-80 bg-white shadow-md shadow-indigo-600/30 transition hover:shadow-lg hover:shadow-indigo-600/20 hover:-translate-y-px"
>
	{#if visible}
		<ul class="max-h-64 overflow-auto py-2">
			{#each candidates as candidate}
				<li>
					<div
						role="button"
						class="block px-4 py-2 text-sm text-gray-800 hover:bg-gray-100"
						on:click={() => loadRiver(candidate)}
						on:keydown={(ev) => onLoadRiverKeydown(ev, candidate)}
						tabindex="-1"
					>
						{candidate.river_name}:{candidate.river_code} ({candidate.prefecture_name})
					</div>
				</li>
			{/each}
		</ul>
	{/if}
</div>
