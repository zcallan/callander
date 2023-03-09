<script lang="ts">
  import dayjs from 'dayjs';
  import type {
    Friend,
    FriendsIdea,
    MetAtAccuracyEnum,
    NewFriendsIdea,
    UpdateFriend,
  } from '$lib/types/generated';
  import { createMutation } from '@tanstack/svelte-query';
  import { updateFriend } from '$lib/api/friends';
  import { serializeForm } from '$lib/utils/serializeForm';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { getFriendByIdQuery } from '$lib/queries/friends';
  import { createFriendIdeaMutation, updateFriendMutation } from '$lib/mutations/friends';
  import Input from '$lib/components/Input.svelte';
  import range from '$lib/utils/range';
  import { FriendIdeaType } from '$lib/types/friends';
  import toSentenceCase from '$lib/utils/toSentenceCase';

  export let friendId: string;
  export let onSuccess: (newFriendIdea: FriendsIdea) => void;

  const updateMutation = createFriendIdeaMutation({
    onSuccess: (updatedFriend: FriendsIdea) => {
      onSuccess(updatedFriend);
    },
  });

  function handleSubmitCreateFriend(event: any) {
    const data = serializeForm(event.target);

    const newFriendIdea: NewFriendsIdea = {
      friend_id: friendId,
      idea_type: data.idea_type,
      content: data.content,
    };

    $updateMutation.mutate(newFriendIdea);
  }

  const ideaTypeOptions = Object.values(FriendIdeaType).map((type) => ({
    value: type,
    label: toSentenceCase(type),
  }));
</script>

<div class="wrapper">
  <h2>Add Friend Idea</h2>

  <form on:submit|preventDefault={handleSubmitCreateFriend}>
    <Input label="Type" name="idea_type" type="select" options={ideaTypeOptions} />

    <Input label="Content" name="content" required />

    {#if $updateMutation.isError}
      <p>Error: {$updateMutation.error.message}</p>
    {/if}

    <button type="submit" disabled={$updateMutation.isLoading}>
      {$updateMutation.isLoading ? 'Loading...' : 'Create'}
    </button>
  </form>
</div>

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
