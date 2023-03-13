<script lang="ts">
  import { PUBLIC_HANKO_API_URL } from '$env/static/public';

  import { onDestroy, onMount } from 'svelte';
  import { register } from '@teamhanko/hanko-elements';
  import { goto } from '$app/navigation';

  let element: HTMLElement;

  const redirectAfterLogin = () => {
    // successfully logged in, redirect to a page in your application
    goto('/');
  };

  onMount(async () => {
    // register the component
    // see: https://github.com/teamhanko/hanko/blob/main/frontend/elements/README.md#script
    register({ shadow: true }).catch((error) => {
      // handle error
      console.log({ error });
    });

    element?.addEventListener('hankoAuthSuccess', redirectAfterLogin);
  });

  onDestroy(() => {
    element?.removeEventListener('hankoAuthSuccess', redirectAfterLogin);
  });
</script>

<hanko-auth bind:this={element} api={PUBLIC_HANKO_API_URL} />

<style global>
  /* https://github.com/teamhanko/hanko/tree/main/frontend/elements#ui-customization */
  hanko-auth {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background: rgba(0, 0, 0, 0.3);
    z-index: 999;
  }

  /* hanko-auth::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    filter: blur(4px);
  } */

  ::part(container) {
    padding: 24px;
    width: 100%;
    max-width: 400px;
  }
</style>
