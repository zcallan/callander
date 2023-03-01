<script lang="ts">
	import dayjs from 'dayjs';
	import type { Friend, NewFriend } from '../../../types/generated';
	import { createMutation } from '@tanstack/svelte-query';
	import { createFriend } from '../../../api/friends';
	import { serializeForm } from '$lib/serializeForm';
	import { goto } from '$app/navigation';

	const createFriendMutation = createMutation<Friend, Error, NewFriend>(createFriend, {
		onSuccess: (newFriend) => {
			goto(`/friends/${newFriend.id}`);
		}
	});

	function handleSubmitCreateFriend(event: any) {
		const data = serializeForm(event.target);

		const newFriend: NewFriend = {
			first_name: data.first_name,
			last_name: data.last_name,
			date_of_birth: data.date_of_birth
		};

		$createFriendMutation.mutate(newFriend);
	}
</script>

<svelte:head>
	<title>Create Friend</title>
</svelte:head>

<section>
	<div>
		<a class="button" href="/"> Back </a>
	</div>

	<h1>Create Friend</h1>
	<form on:submit|preventDefault={handleSubmitCreateFriend}>
		<label>
			First Name
			<input type="text" name="first_name" required />
		</label>

		<label>
			Last Name
			<input type="text" name="first_name" required />
		</label>

		<label>
			Date of Birth (optional)
			<input type="text" name="date_of_birth" placeholder="YYYY-MM-DD" />
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
