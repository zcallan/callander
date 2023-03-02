<script lang="ts">
  import { createQuery } from '@tanstack/svelte-query';
  import type { Friend, Post } from '$lib/types/generated';
  import { getFriends } from '$lib/api/friends';
  import { getPosts } from '$lib/api/posts';
  import postsQuery from '$lib/queries/postsQuery';

  // This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
  const friendsQuery = createQuery<Friend[], Error>({
    queryKey: ['friends'],
    queryFn: getFriends,
  });

  const posts = postsQuery({ page: 1, per_page: 10 });
</script>

<svelte:head>
  <title>Callander</title>
  <meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
  <a href="/friends">
    <h2>Friends</h2>
  </a>

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

<hr />

<section>
  <a href="/posts">
    <h2>Posts</h2>
  </a>

  {#if $posts.isLoading}
    <p>Loading...</p>
  {:else if $posts.isError}
    <p>Error: {$posts.error.message}</p>
  {:else if $posts.isSuccess}
    {#each $posts.data.items as post}
      <a href="/posts/{post.id}">{post.title} ({post.id})</a>
    {:else}
      <p>No posts</p>
    {/each}
  {/if}

  <br />

  <a href="/posts/new">Create Post</a>
</section>

<style>
  label {
    display: block;
  }

  a {
    display: block;
    padding: 5px 0;
  }

  section {
    padding: 24px 32px;
  }
</style>
