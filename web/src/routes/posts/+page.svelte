<script lang="ts">
	import type { Post } from '../../types/generated';
	import { createQuery } from '@tanstack/svelte-query';
	import { getPosts } from '../../api/posts';

	// This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
	const postsQuery = createQuery<Post[], Error>({
		queryKey: ['posts'],
		queryFn: getPosts
	});
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
		{#each $postsQuery.data as post}
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
