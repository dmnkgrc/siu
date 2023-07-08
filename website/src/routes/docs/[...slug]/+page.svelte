<script lang="ts">
  import type { PageData } from "./$types";
  
  export let data: PageData;
</script>

<svelet:head>
  <title>
    {data.page.frontmatter.title} • Docs • smu
  </title>
  <meta name="Description" content="{data.page.frontmatter.title} • smu documentation" />
</svelet:head>

<div>
  <h1 class="h1 pb-10">{data.page.frontmatter.title}</h1>
  <article class="prose lg:prose-xl prose-stone">
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html data.page.content}
	</article>
</div>
