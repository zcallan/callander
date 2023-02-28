<script lang="ts">
	import type { NewFriend, Friend, Post, NewPost } from '../types/generated';
	import { createQuery, createMutation } from '@tanstack/svelte-query';
	import { getFriends, createFriend } from '../api/friends';
	import type { LayoutData } from './$types';
	import { createPost, getPosts } from '../api/posts';

	export let data: LayoutData;

	let createFriendForm: HTMLFormElement;
	let createPostForm: HTMLFormElement;

	// This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
	const friendsQuery = createQuery<Friend[], Error>({
		queryKey: ['friends'],
		queryFn: getFriends
	});

	const postsQuery = createQuery<Post[], Error>({
		queryKey: ['posts'],
		queryFn: getPosts
	});

	const createFriendMutation = createMutation<Friend, Error, NewFriend>(createFriend, {
		onMutate: async (newFriend: NewFriend) => {
			await data.queryClient.cancelQueries(['friends']);

			// Snapshot the previous value
			const previousFriends = data.queryClient.getQueryData<Friend[]>(['friends']);

			// Optimistically update to the new value
			if (previousFriends) {
				data.queryClient.setQueryData<Friend[]>(
					['friends'],
					[...previousFriends, { id: Math.random().toString(), ...newFriend }]
				);
			}

			return { previousFriends };
		},
		onError: (err: Error, variables: NewFriend, context: any) => {
			if (context?.previousFriends) {
				data.queryClient.setQueryData<Friend[]>(['friends'], context.previousFriends);
			}
		},
		onSuccess: () => {
			createFriendForm.reset();
		},
		// Always refetch after error or success:
		onSettled: () => {
			data.queryClient.invalidateQueries(['friends']);
		}
	});

	const createPostMutation = createMutation<Post, Error, NewPost>(createPost, {
		onSuccess: () => {
			createPostForm.reset();
			$postsQuery.refetch();
		}
	});

	const serializeForm = (form: HTMLFormElement) => {
		const formData = new FormData(form);
		const data: any = {};

		for (let field of formData) {
			const [key, value] = field;
			data[key] = value;
		}

		return data;
	};

	function handleSubmitCreateFriend(event: any) {
		const data = serializeForm(event.target);

		const newFriend: NewFriend = {
			first_name: data.first_name,
			last_name: data.last_name,
			date_of_birth: data.date_of_birth
		};

		$createFriendMutation.mutate(newFriend);
	}

	function handleSubmitCreatePost(event: any) {
		const data = serializeForm(event.target);

		const newPost: NewPost = {
			title: data.title,
			body: data.body,
			for_date: data.for_date
		};

		$createPostMutation.mutate(newPost);
	}
</script>

<svelte:head>
	<title>Callander</title>
	<meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
	<h2>Friends</h2>
	{#if $friendsQuery.isLoading}
		<p>Loading...</p>
	{:else if $friendsQuery.isError}
		<p>Error: {$friendsQuery.error.message}</p>
	{:else if $friendsQuery.isSuccess}
		{#each $friendsQuery.data as friend}
			<p>{friend.first_name} {friend.last_name}</p>
		{:else}
			<p>No friends</p>
		{/each}
	{/if}

	<form on:submit|preventDefault={handleSubmitCreateFriend} bind:this={createFriendForm}>
		<label>
			First name
			<input type="text" name="first_name" required />
		</label>

		<label>
			Last name
			<input type="text" name="last_name" required />
		</label>

		{#if $createFriendMutation.isError}
			<p>Error: {$createFriendMutation.error.message}</p>
		{/if}

		<button type="submit" disabled={$createFriendMutation.isLoading}> Submit </button>
	</form>
</section>

<hr />

<section>
	<h2>Posts</h2>
	{#if $postsQuery.isLoading}
		<p>Loading...</p>
	{:else if $postsQuery.isError}
		<p>Error: {$postsQuery.error.message}</p>
	{:else if $postsQuery.isSuccess}
		{#each $postsQuery.data as post}
			<p>{post.title}</p>
		{:else}
			<p>No posts</p>
		{/each}
	{/if}

	<form on:submit|preventDefault={handleSubmitCreatePost} bind:this={createPostForm}>
		<label>
			Title
			<input type="text" name="title" required />
		</label>

		<label>
			Body
			<textarea name="body" required />
		</label>

		<label>
			For Date
			<input type="text" name="for_date" required placeholder="YYYY-MM-DD" />
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
