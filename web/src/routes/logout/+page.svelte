<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  const deleteCookie = (name: string) => {
    // document.cookie = name + '=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;';
  };

  onMount(async () => {
    // deleteCookie('hanko');
    // localStorage.removeItem('hanko');
    alert('TODO');
    goto('/');
  });
</script>

<p>Logging out...</p>
