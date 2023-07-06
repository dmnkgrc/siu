<script lang="ts">
  import type { PageData } from "./$types";
  
  export let data: PageData;
</script>

<svelet:head>
  <title>
    {data.page.data.title} • Docs • smu
  </title>
  <meta name="Description" content="{data.page.data.title} • smu documentation" />
</svelet:head>

<div>
  <h1 class="h1 pb-10">{data.page.data.title}</h1>
  <section>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html data.page.content}
	</section>
</div>
