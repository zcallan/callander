<script lang="ts">
	import dayjs from 'dayjs';
	import type { Post, NewPost } from '../../../types/generated';
	import { createMutation } from '@tanstack/svelte-query';
	import { createPost } from '../../../api/posts';
	import Editor from '../../../components/Editor.svelte';
	import { serializeForm } from '$lib/serializeForm';
	import { goto } from '$app/navigation';

	let content: string;

	const createPostMutation = createMutation<Post, Error, NewPost>(createPost, {
		onSuccess: (newPost) => {
			// createPostForm.reset();
			goto(`/posts/${newPost.id}`);
		}
	});

	function handleSubmitCreatePost(event: any) {
		const data = serializeForm(event.target);

		const newPost: NewPost = {
			title: data.title,
			body: content,
			for_date: data.for_date
		};

		$createPostMutation.mutate(newPost);
	}
</script>

<svelte:head>
	<title>Create Post</title>
</svelte:head>

<section>
	<div>
		<a class="button" href="/"> Back </a>
	</div>
	<h1>Create Post</h1>
	<form on:submit|preventDefault={handleSubmitCreatePost}>
		<label>
			Title
			<input type="text" name="title" required />
		</label>

		<label> Body </label>
		<Editor bind:content />

		<label>
			For Date
			<input
				type="text"
				name="for_date"
				required
				placeholder="YYYY-MM-DD"
				value={dayjs().format('YYYY-MM-DD')}
			/>
		</label>

		{#if $createPostMutation.isError}
			<p>Error: {$createPostMutation.error.message}</p>
		{/if}

		<button type="submit" disabled={$createPostMutation.isLoading}>
			{$createPostMutation.isLoading ? 'Loading...' : 'Submit'}
		</button>
	</form>
</section>

<style>
	label {
		display: block;
	}
</style>
