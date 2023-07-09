<script lang="ts">
	import type { Node } from './types';

	export let children: Node[] | undefined = undefined;
	export let name: string;
	export let isActive = false;
	export let url: string;

	$: classesActive = (isActive: boolean) =>
		isActive
			? 'border-primary-500 text-primary-500 text-semibold'
			: 'border-slate-700 dark:hover:border-slate-500 dark:text-slate-400 dark:hover:text-slate-300';
</script>

<li class="flex flex-col space-y-4">
	{#if children}
		<span class="flex-auto font-bold capitalize">{name}</span>
		<ul>
			{#each children as child}
				<svelte:self {...child} />
			{/each}
		</ul>
	{:else}
		<a href={url} class="border-l-2 px-2 capitalize {classesActive(isActive)}">
			{name}
		</a>
	{/if}
</li>
