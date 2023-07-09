<script lang="ts">
	import { AppShell, Drawer, drawerStore } from '@skeletonlabs/skeleton';
	import { Cross2, HamburgerMenu } from '@steeze-ui/radix-icons';
	import { Icon } from '@steeze-ui/svelte-icon';

	import type { PageServerData } from './$types';
	import SidebarNode from './SidebarNode.svelte';

	export let data: PageServerData;
	$: classesActive = (index: number) =>
		index === data.crumbs.length - 1 ? 'text-semibold' : 'text-slate-500 dark:text-slate-400';
</script>

<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<div id="sidebar-left" class="fixed hidden w-64 lg:block">
			<nav class="p-4">
				<ul class="flex flex-col space-y-4">
					{#each data.structure as node}
						<SidebarNode {...node} />
					{/each}
				</ul>
			</nav>
		</div>
	</svelte:fragment>
	<Drawer width="w-64">
		<div class="flex flex-row items-center justify-between p-4 text-slate-500 dark:text-slate-400">
			<p class="font-semibold">Documentation</p>
			<button on:click={() => drawerStore.close()}>
				<Icon src={Cross2} class="h-5 w-5" />
			</button>
		</div>
		<nav class="p-4">
			<ul class="flex flex-col space-y-4">
				{#each data.structure as node}
					<SidebarNode {...node} />
				{/each}
			</ul>
		</nav>
	</Drawer>
	<div
		class="large:hidden fixed flex w-full flex-row items-center space-x-4 border-b border-slate-900/10 bg-surface-50 px-4 py-4 dark:border-slate-50/[0.06] dark:bg-surface-900 lg:hidden"
	>
		<button on:click={() => drawerStore.open()}>
			<Icon
				src={HamburgerMenu}
				class="h-6 w-6 text-slate-500 hover:text-slate-400 dark:text-slate-400 dark:hover:text-slate-300"
			/>
		</button>
		<ol class="flex h-full flex-row items-center space-x-2">
			{#each data.crumbs as item, index}
				<li class="text-sm capitalize {classesActive(index)}">{item}</li>
				{#if index < data.crumbs.length - 1}
					<li class="crumb-separator" aria-hidden>&rsaquo;</li>
				{/if}
			{/each}
		</ol>
	</div>
	<main class="h-full w-full p-4 pt-24 lg:p-16 lg:pl-[19rem]">
		<slot />
	</main>
</AppShell>
