<script lang="ts">
	import type { Friend } from '../../types/generated';
	import { createQuery } from '@tanstack/svelte-query';
	import { getFriends } from '../../api/friends';

	// This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
	const friendsQuery = createQuery<Friend[], Error>({
		queryKey: ['friends'],
		queryFn: getFriends
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
	a {
		display: block;
		padding: 5px 0;
	}
</style>
