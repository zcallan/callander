<script lang="ts">
  import { PUBLIC_HANKO_API_URL } from '$env/static/public';

  import { onDestroy, onMount } from 'svelte';
  import { register } from '@teamhanko/hanko-elements';

  let element: HTMLElement;

  const redirectAfterLogin = () => {
    // successfully logged in, redirect to a page in your application
    console.log('redirect');
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
