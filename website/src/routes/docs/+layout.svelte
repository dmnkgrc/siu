<script lang="ts">
	import { AppShell } from '@skeletonlabs/skeleton';
</script>

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="sidebarLeft">
		<div id="sidebar-left" class="hidden lg:block">Sidebar</div>
	</svelte:fragment>
	<!-- Page Route Content -->
  <main class="w-full p-16">
    <slot />
  </main>
</AppShell>
