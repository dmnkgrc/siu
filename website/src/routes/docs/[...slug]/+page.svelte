<script lang="ts">
	import { AppShell } from '@skeletonlabs/skeleton';

	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelet:head>
	<title>
		{data.metadata.title} • Docs • smu
	</title>
	<meta name="Description" content="{data.metadata.title} • smu documentation" />
</svelet:head>

<AppShell>
	<svelte:fragment slot="sidebarLeft">
		<div id="sidebar-left" class="hidden lg:block">Sidebar</div>
	</svelte:fragment>
	<main class="w-full p-16">
		<h1 class="h1 pb-10">{data.metadata.title}</h1>
		<div class="prose">
			<svelte:component this={data.content} />
		</div>
	</main>
</AppShell>
