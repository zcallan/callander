<script lang="ts">
	import { onMount } from 'svelte';
	import type { CreateFriendDto, Friend } from '../types/friend.types';

	let friends: Friend[] = [];

	async function getFriends() {
		const res = await fetch(`http://localhost:8080/friends`);
		friends = await res.json();
	}

	async function createFriend(data: CreateFriendDto) {
		try {
			await fetch(`http://localhost:8080/friends`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(data)
			});

			getFriends();
		} catch (error) {
			console.error(error);
		}
	}

	onMount(() => {
		getFriends();
	});

	function handleSubmit(event: any) {
		const formData = new FormData(event.target);
		const data: any = {};

		for (let field of formData) {
			const [key, value] = field;
			data[key] = value;
		}

		createFriend(data);
	}
</script>

<svelte:head>
	<title>Callander</title>
	<meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
	<h2>Friends</h2>
	{#each friends as friend}
		<div>
			{friend.first_name}
			{friend.last_name}
		</div>
	{/each}

	<form on:submit|preventDefault={handleSubmit}>
		<label>
			First name
			<input type="text" name="first_name" required />
		</label>

		<label>
			Last name
			<input type="text" name="last_name" required />
		</label>

		<button type="submit">Submit</button>
	</form>
</section>

<style>
</style>
