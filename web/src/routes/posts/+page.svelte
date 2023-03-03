<script lang="ts">
  import { getPostsQuery } from '$lib/queries/posts';

  let page = 1;
  let perPage = 10;

  // This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
  const postsQuery = getPostsQuery({ page, per_page: perPage });
</script>

<svelte:head>
  <title>Callander</title>
  <meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
  <div>
    <a class="button" href="/"> Back </a>
  </div>
  <h2>Posts</h2>
  {#if $postsQuery.isLoading}
    <p>Loading...</p>
  {:else if $postsQuery.isError}
    <p>Error: {$postsQuery.error.message}</p>
  {:else if $postsQuery.isSuccess}
    {#each $postsQuery.data.items as post}
      <a href="/posts/{post.id}">{post.title}</a>
    {:else}
      <p>No posts</p>
    {/each}
  {/if}

  <br />

  <a href="/posts/new">Create Post</a>
</section>

<style>
  a {
    display: block;
    padding: 5px 0;
  }
</style>
