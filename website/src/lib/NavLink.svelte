<script lang="ts">
	export let href: string;
	export let target: string | undefined = undefined;
	export let rel: string | undefined = undefined;
</script>

<a {href} {target} {rel} class="btn btn-sm variant-ghost-surface flex items-center gap-3"
	><slot /></a
>
