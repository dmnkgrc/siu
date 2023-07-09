<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelet:head>
	<title>
		{data.metadata.title} • Docs • smu
	</title>
	<meta name="Description" content="{data.metadata.title} • smu documentation" />
</svelet:head>

<h1 class="h1 pb-10 font-extrabold">{data.metadata.title}</h1>
<div class="prose">
	<svelte:component this={data.content} />
</div>
