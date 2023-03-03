<script lang="ts">
  import dayjs from 'dayjs';
  import type { Friend, NewFriend } from '$lib/types/generated';
  import { createMutation } from '@tanstack/svelte-query';
  import { createFriend } from '$lib/api/friends';
  import { serializeForm } from '$lib/utils/serializeForm';
  import { goto } from '$app/navigation';
  import Input from '$lib/components/Input.svelte';

  let dateOfBirth: Date;

  const createFriendMutation = createMutation<Friend, Error, NewFriend>(createFriend, {
    onSuccess: (newFriend) => {
      goto(`/friends/${newFriend.id}`);
    },
  });

  function handleSubmitCreateFriend(event: any) {
    const data = serializeForm(event.target);

    const newFriend: NewFriend = {
      first_name: data.first_name,
      last_name: data.last_name,
      date_of_birth: data.date_of_birth,
      met_at: data.met_at,
      met_at_accuracy: data.met_at_accuracy,
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
    <Input label="First Name" name="first_name" required />

    <Input label="Last Name" name="last_name" required />

    <Input label="Date of Birth" type="date" name="date_of_birth" bind:date={dateOfBirth} />

    <Input label="Met At" type="date" name="met_at" />

    {#if $createFriendMutation.isError}
      <p>Error: {$createFriendMutation.error.message}</p>
    {/if}

    <button type="submit" disabled={$createFriendMutation.isLoading}>
      {$createFriendMutation.isLoading ? 'Loading...' : 'Submit'}
    </button>
  </form>
</section>

<style>
  :global(.input) {
    margin-bottom: 16px;
  }

  form {
    max-width: 200px;
    width: 100%;
    margin: 0 auto;
  }

  button[type='submit'] {
    width: 100%;
  }
</style>
