<script lang="ts">
  import Friend from '$lib/components/Friend.svelte';
  import type { PageData } from './$types';

  export let data: PageData;

  const friendId: string = data.friendId;
</script>

<Friend {friendId} />
