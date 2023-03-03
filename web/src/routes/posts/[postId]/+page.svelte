<script lang="ts">
  import { getPostByIdQuery } from '$lib/queries/posts';
  import type { PageData } from './$types';

  export let data: PageData;

  const postId: string = data.postId;

  const post = getPostByIdQuery(postId);
</script>

<div class="wrapper">
  <div>
    <a class="button" href="/"> Back </a>
  </div>
  {#if !postId || $post.isLoading}
    <span>Loading...</span>
  {/if}
  {#if $post.error}
    <span>Error: {$post.error.message}</span>
  {/if}
  {#if $post.isSuccess}
    <h1>{$post.data.title}</h1>
    <div>
      {@html $post.data.body}
    </div>
    <div>{$post.isFetching ? 'Background Updating...' : ' '}</div>
  {/if}
</div>

<style>
  .wrapper {
    text-align: center;
    padding: 40px;
  }
</style>
