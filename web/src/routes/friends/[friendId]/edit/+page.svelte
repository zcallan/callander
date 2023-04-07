<script lang="ts">
  import type { Friend, UpdateFriend } from '$lib/types/generated';
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import { getFriendByIdQuery } from '$lib/queries/friends';
  import { updateFriendMutation } from '$lib/mutations/friends';
  import FriendForm from '$lib/components/FriendForm.svelte';

  export let data: PageData;

  const friendId: string = data.friendId;

  const friendQuery = getFriendByIdQuery(friendId);

  const updateMutation = updateFriendMutation(friendId, {
    onSuccess: (updatedFriend: Friend) => {
      goto(`/friends/${updatedFriend.id}`);
    },
  });

  function handleSubmit(values: UpdateFriend) {
    $updateMutation.mutate(values);
  }
</script>

<section>
  <div>
    <a class="button" href="/"> Back </a>
  </div>

  {#if $friendQuery.isLoading}
    <p>Loading...</p>
  {:else if $friendQuery.isError}
    <p>Error: {$friendQuery.error.message}</p>
  {:else if !$friendQuery.data}
    <p>Unable to load friend</p>
  {:else}
    <h2>Edit {$friendQuery.data.first_name} {$friendQuery.data.last_name}</h2>

    <FriendForm
      isEditing
      defaultValues={$friendQuery.data}
      onSubmit={handleSubmit}
      isLoading={$updateMutation.isLoading}
      errorMessage={$updateMutation.error?.message}
    />
  {/if}
</section>

<style>
  section {
    padding: 40px;
  }
</style>
