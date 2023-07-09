<script lang="ts">
	import { AppShell } from '@skeletonlabs/skeleton';

	import type { PageServerData } from './$types';
	import SidebarNode from './SidebarNode.svelte';

	export let data: PageServerData;
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
	<main class="h-full w-full p-16 lg:pl-[19rem]">
		<slot />
	</main>
</AppShell>
