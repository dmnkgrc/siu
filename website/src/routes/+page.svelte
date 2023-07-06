<script>
	import FeatureCard from '$lib/FeatureCard.svelte';
	import logo from '$lib/assets/logo.svg';
	import { CodeBlock } from '@skeletonlabs/skeleton';
	import { ChevronRight, Gear, Layers, Mix, Rocket } from '@steeze-ui/radix-icons';
	import { Icon } from '@steeze-ui/svelte-icon';
</script>

<div class="container mx-auto flex h-full flex-col items-center">
	<div class="flex flex-col items-center pb-10 text-center">
		<div class="flex w-full items-center justify-center py-10">
			<img src={logo} alt="smu logo" class="w-72" />
		</div>
		<h1 class="h2 text-primary-400">smu</h1>
		<h2 class="h3">Automated setup of the future</h2>
		<div class="flex justify-center p-8">
			<a
				class="btn variant-filled animate-bounce space-y-10"
				href="/docs/guides/getting-started"
				target="_blank"
				rel="noreferrer"
			>
				Get started
				<Icon src={ChevronRight} class="ml-2 w-6" />
			</a>
		</div>
	</div>
	<div class="grid w-full grid-cols-1 gap-4 px-4 md:grid-cols-2">
		<FeatureCard title="Getting started" icon={Rocket} iconColor="primary">
			Check out the <a class="text-pink-400" href="/docs/guides/getting-started"
				>Getting started guide</a
			>
			for a quick introduction to the basics and write your first configuration.

			<CodeBlock
				class="mt-4"
				language="yaml"
				code={`name: Personal computer setup
description: Setup my personal computer to work on side projects
steps:
  - description: "Install Node related packages"
    run:
      - brew: fnm pnpm
  - description: Install development tools
    run:
      - chezmoi:
          repo: dmnkgrc/dotfiles
          apply: true
      - brew: neovim tmux zellij zoxide bat lsd git gh
      - pnpm: neovim
        `}
			/>
		</FeatureCard>
		<FeatureCard title="Configuration" icon={Gear} iconColor="blue">
			You can read over the <a class="text-pink-400" href="/docs/configuration"
				>configuration section</a
			> for more information on all the available options.
		</FeatureCard>
		<FeatureCard title="Integrations" icon={Layers} iconColor="orange">
			The <a class="text-pink-400" href="/docs/integrations">integrations section</a> contains a list
			of all the current integrations and what they can do.
		</FeatureCard>
		<FeatureCard title="Examples" icon={Mix} iconColor="red">
			Take a look at the <a class="text-pink-400" href="/docs/guides/examples">examples</a> for inspiration.
		</FeatureCard>
	</div>
</div>
