<script lang="ts">
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';

	import { Icon } from '@steeze-ui/svelte-icon';
	import { GithubLogo } from '@steeze-ui/radix-icons';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import NavLink from '$lib/NavLink.svelte';
	import logo from '$lib/assets/logo.svg';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/tokyo-night-dark.css';

	import { storeHighlightJs } from '@skeletonlabs/skeleton';

	storeHighlightJs.set(hljs);
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<div class="flex h-6 items-center space-x-4">
					<img src={logo} alt="smu logo" class="w-10" />
					<strong class="text-xl">smu</strong>
				</div>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<NavLink href="/docs">Documentation</NavLink>
				<NavLink href="https://github.com/dmnkgrc/smu" target="_blank" rel="noreferrer">
					<Icon src={GithubLogo} class="h-6 w-6" />
				</NavLink>
				<LightSwitch />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
