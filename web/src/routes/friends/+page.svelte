<script lang="ts">
  import { getFriendsQuery } from '$lib/queries/friends';

  // This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
  const friendsQuery = getFriendsQuery({ page: 1, per_page: 10 });
</script>

<svelte:head>
  <title>Callander</title>
  <meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
  <div>
    <a class="button" href="/"> Back </a>
  </div>
  <h2>Friends</h2>
  {#if $friendsQuery.isLoading}
    <p>Loading...</p>
  {:else if $friendsQuery.isError}
    <p>Error: {$friendsQuery.error.message}</p>
  {:else if $friendsQuery.isSuccess}
    {#each $friendsQuery.data as friend}
      <a href="/friends/{friend.id}">{friend.first_name} {friend.last_name}</a>
    {:else}
      <p>No friends</p>
    {/each}
  {/if}

  <br />

  <a href="/friends/new">Create Friend</a>
</section>

<style>
  section {
    padding: 24px 32px;
  }

  a {
    display: block;
    padding: 5px 0;
  }
</style>
