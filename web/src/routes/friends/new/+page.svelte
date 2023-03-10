<script lang="ts">
  import type { Friend, NewFriend } from '$lib/types/generated';
  import { createMutation } from '@tanstack/svelte-query';
  import { createFriend } from '$lib/api/friends';
  import { goto } from '$app/navigation';
  import FriendForm from '$lib/components/FriendForm.svelte';

  const createFriendMutation = createMutation<Friend, Error, NewFriend>(createFriend, {
    onSuccess: (newFriend) => {
      goto(`/friends/${newFriend.id}`);
    },
  });

  function handleSubmit(values: NewFriend) {
    $createFriendMutation.mutate(values);
  }
</script>

<svelte:head>
  <title>Create Friend</title>
</svelte:head>

<section>
  <div>
    <a class="button" href="/"> Back </a>
  </div>

  <h2>Create Friend</h2>
  <FriendForm
    onSubmit={handleSubmit}
    isLoading={$createFriendMutation.isLoading}
    errorMessage={$createFriendMutation.error?.message}
  />
</section>

<style>
  :global(.input) {
    margin-bottom: 16px;
  }

  form {
    max-width: 200px;
    width: 100%;
  }

  button[type='submit'] {
    width: 100%;
  }

  section {
    padding: 40px;
  }
</style>
