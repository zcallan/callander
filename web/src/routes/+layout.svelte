<script lang="ts">
  import './styles.css';
  import Header from '$lib/components/Header.svelte';
  import { QueryClientProvider } from '@tanstack/svelte-query';
  import type { LayoutData } from './$types';
  import Footer from '$lib/components/Footer.svelte';
  import HankoAuth from '$lib/components/HankoAuth.svelte';
  import { isAuthenticated } from '$lib/stores';
  import cookie from 'cookie';
  import { browser } from '$app/environment';

  export let data: LayoutData;

  /** TODO: Better way of doing this? Any point to using SvelteKit? */
  if (browser) {
    const cookies = cookie.parse(document.cookie);
    if (cookies?.hanko) isAuthenticated.set(true);
  }
</script>

{#if !browser}
  <p class="loading">Loading...</p>
{:else if !$isAuthenticated}
  <HankoAuth />
{:else}
  <QueryClientProvider client={data.queryClient}>
    <div class="app">
      <Header />

      <main>
        <slot />
      </main>
    </div>

    <Footer />
  </QueryClientProvider>
{/if}

<style>
  .app {
  }

  .loading {
    text-align: center;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>
