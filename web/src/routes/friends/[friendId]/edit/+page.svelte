<script lang="ts">
  import dayjs from 'dayjs';
  import type { Friend, UpdateFriend } from '$lib/types/generated';
  import { createMutation } from '@tanstack/svelte-query';
  import { updateFriend } from '$lib/api/friends';
  import { serializeForm } from '$lib/utils/serializeForm';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import type { PageData } from './$types';
  import { getFriendByIdQuery } from '$lib/queries/friends';
  import { updateFriendMutation } from '$lib/mutations/friends';
  import { DateInput } from 'date-picker-svelte';
  import getDayOfYear from '$lib/utils/getDayOfYear';
  import Input from '$lib/components/Input.svelte';

  export let data: PageData;

  const friendId: string = data.friendId;

  const friendQuery = getFriendByIdQuery(friendId);

  let dateOfBirth: Date;
  let metAt: Date;

  friendQuery.subscribe((friend) => {
    if (friend.data?.date_of_birth) dateOfBirth = new Date(friend.data.date_of_birth);
    if (friend.data?.met_at) metAt = new Date(friend.data.met_at);
  });

  const updateMutation = updateFriendMutation(friendId, {
    onSuccess: (updatedFriend: Friend) => {
      goto(`/friends/${updatedFriend.id}`);
    },
  });

  function handleSubmitCreateFriend(event: any) {
    const data = serializeForm(event.target);

    const _dateOfBirth = dateOfBirth ? dayjs(dateOfBirth).format('YYYY-MM-DD') : undefined;
    const _metAt = metAt ? dayjs(metAt).format('YYYY-MM-DD') : undefined;

    const updatedFriend: UpdateFriend = {
      first_name: data.first_name,
      last_name: data.last_name,
      date_of_birth: _dateOfBirth,
      met_at: _metAt,
    };

    $updateMutation.mutate(updatedFriend);
  }
</script>

<svelte:head>
  <title>Create Friend</title>
</svelte:head>

<section>
  <div>
    <a class="button" href="/friends/{friendId}"> Back </a>
  </div>

  {#if $friendQuery.isLoading}
    <p>Loading...</p>
  {:else if $friendQuery.isError}
    <p>Error: {$friendQuery.error.message}</p>
  {:else}
    <h1>Edit {$friendQuery.data.first_name} {$friendQuery.data.last_name}</h1>

    <form on:submit|preventDefault={handleSubmitCreateFriend}>
      <Input
        label="First Name"
        name="first_name"
        required
        value={$friendQuery.data.first_name ?? ''}
      />

      <Input
        label="Last Name"
        name="last_name"
        required
        value={$friendQuery.data.last_name ?? ''}
      />

      <Input
        label="Date of Birth"
        type="date"
        name="date_of_birth"
        required
        bind:date={dateOfBirth}
      />

      <Input label="Met At" type="date" name="met_at" bind:date={metAt} />

      {#if $updateMutation.isError}
        <p>Error: {$updateMutation.error.message}</p>
      {/if}

      <button type="submit" disabled={$updateMutation.isLoading}>
        {$updateMutation.isLoading ? 'Loading...' : 'Update'}
      </button>
    </form>
  {/if}
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
