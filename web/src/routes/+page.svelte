<script lang="ts">
  import type { Friend, Post } from '../types/generated';
  import { createQuery } from '@tanstack/svelte-query';
  import { getFriends } from '../api/friends';
  import { getPosts } from '../api/posts';
  import postsQuery from '../queries/postsQuery';

  // This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
  const friendsQuery = createQuery<Friend[], Error>({
    queryKey: ['friends'],
    queryFn: getFriends,
  });

  const posts = postsQuery({ limit: 2 });
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
    {#each $posts.data as post}
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
