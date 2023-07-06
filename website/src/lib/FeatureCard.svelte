<script lang="ts">
	import type { IconSource } from '@steeze-ui/radix-icons';
	import { Icon } from '@steeze-ui/svelte-icon';

	export let title: string;
	export let icon: IconSource;
	export let iconColor: 'primary' | 'secondary' | 'red' | 'blue' | 'pink' | 'orange';
</script>

<div class="card variant-ringed">
	<header class="card-header p-0">
		<div class="card variant-ghost flex items-center space-x-4 p-4">
			<div class="rounded bg-{iconColor}-500 p-2">
				<Icon src={icon} class="w-5" />
			</div>
			<p>{title}</p>
		</div>
		<section class="p-4"><slot /></section>
	</header>
</div>
