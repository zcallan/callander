<script lang="ts">
	import './styles.css';
	import Header from '../components/Header.svelte';
	import { QueryClientProvider } from '@tanstack/svelte-query';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<QueryClientProvider client={data.queryClient}>
	<div class="app">
		<Header />

		<main>
			<slot />
		</main>
	</div>
</QueryClientProvider>

<style>
</style>
