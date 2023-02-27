<script lang="ts">
	import type { NewFriend, Friend } from '../types/generated';
	import { createQuery, createMutation } from '@tanstack/svelte-query';
	import { getFriends, createFriend } from '../api/friends';
	import type { LayoutData } from './$types';

	export let data: LayoutData;

	let form: HTMLFormElement;

	// This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
	const friendsQuery = createQuery<Friend[], Error>({
		queryKey: ['friends'],
		queryFn: getFriends
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
			form.reset();
		},
		// Always refetch after error or success:
		onSettled: () => {
			data.queryClient.invalidateQueries(['friends']);
		}
	});

	function handleSubmit(event: any) {
		const formData = new FormData(event.target);
		const data: any = {};

		for (let field of formData) {
			const [key, value] = field;
			data[key] = value;
		}

		const newFriend: NewFriend = {
			first_name: data.first_name,
			last_name: data.last_name,
			date_of_birth: data.date_of_birth
		};

		$createFriendMutation.mutate(newFriend);
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

	<form on:submit|preventDefault={handleSubmit} bind:this={form}>
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

		<button type="submit" disabled={$createFriendMutation.isLoading}>
			{$createFriendMutation.isLoading ? 'Loading...' : 'Submit'}
		</button>
	</form>
</section>

<style>
	label {
		display: block;
	}
</style>
