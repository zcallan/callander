<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { getFriendById } from '../api/friends';
	import type { Friend } from '../types/generated';

	export let friendId: string;

	const friend = createQuery<Friend, Error>({
		queryKey: ['friend', friendId],
		queryFn: () => getFriendById(friendId)
	});
</script>

<div class="wrapper">
	<div>
		<a class="button" href="/"> Back </a>
	</div>
	{#if !friendId || $friend.isLoading}
		<span>Loading...</span>
	{/if}
	{#if $friend.error}
		<span>Error: {$friend.error.message}</span>
	{/if}
	{#if $friend.isSuccess}
		<h1>{$friend.data.first_name} {$friend.data.last_name}</h1>
		<div>
			<b>Date of birth:</b>
			{$friend.data.date_of_birth || 'Not set'}
		</div>
		<div>{$friend.isFetching ? 'Background Updating...' : ' '}</div>
	{/if}
</div>

<style>
	.wrapper {
		text-align: center;
		padding: 40px;
	}
</style>
