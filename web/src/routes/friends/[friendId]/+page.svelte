<script lang="ts">
  import { createQuery } from '@tanstack/svelte-query';
  import type { PageData } from './$types';
  import { getFriendById } from '$lib/api/friends';
  import type { Friend, FriendIdeaTypeEnum, FriendsIdea } from '$lib/types/generated';
  import dayjs from 'dayjs';
  import { getFriendByIdQuery, getFriendIdeasQuery } from '$lib/queries/friends';
  import FriendIdeaForm from '$lib/components/FriendIdeaForm.svelte';

  const CURRENT_YEAR = new Date().getFullYear();
  const BDAY_RECENT_THRESHOLD = -60;

  export let data: PageData;

  const friendId: string = data.friendId;

  const friend = getFriendByIdQuery(friendId);
  const friendIdeas = getFriendIdeasQuery(friendId);

  let attributes: any[] = [];
  let isShowingFriendIdeaForm = false;

  $: attributes = [
    {
      label: 'Date of birth',
      value: $friend.data?.date_of_birth,
      format: (value: any) => {
        if (!value) return null;
        const dateOfBirth = dayjs(value);
        const formattedDob = dayjs(value).format('D MMM YYYY');
        const daysUntilBday = dateOfBirth.year(CURRENT_YEAR).isAfter(dayjs())
          ? dateOfBirth.year(CURRENT_YEAR).diff(dayjs(), 'days')
          : dateOfBirth.year(CURRENT_YEAR + 1).diff(dayjs(), 'days');
        const yearsOld = dayjs().diff(dateOfBirth, 'years');
        return `${formattedDob} (${yearsOld})`;
      },
    },
    {
      label: 'Birthday',
      value: $friend.data?.date_of_birth,
      format: (value: any) => {
        if (!value) return null;
        const birthday = dayjs(value).set('year', CURRENT_YEAR);
        let diff = birthday.diff(dayjs(), 'days');
        if (diff < 0) {
          if (diff > BDAY_RECENT_THRESHOLD) return `${Math.abs(diff)} days ago`;
          diff = birthday.add(1, 'year').diff(dayjs(), 'days');
        }
        return `${diff} days from now`;
      },
    },
    {
      label: 'When we met',
      value: $friend.data?.met_at,
      format: (value: any) => {
        if (!value) return null;
        const metAt = dayjs(value);
        const accuracy = $friend.data?.met_at_accuracy;
        if (accuracy === 'year') return metAt.format('YYYY');
        if (accuracy === 'month') return metAt.format('MMM YYYY');
        if (accuracy === 'day') return metAt.format('D MMM YYYY');
        return metAt.format('D MMM YYYY');
      },
    },
  ];

  $: friendIdeaGroups = $friendIdeas.data?.reduce(
    (groups: { [key in FriendIdeaTypeEnum]?: FriendsIdea[] }, idea: FriendsIdea) => {
      groups[idea.idea_type] = [...(groups[idea.idea_type] || []), idea];
      return groups;
    },
    {}
  );

  console.log(friendIdeaGroups);

  const handleAddedFriendIdea = () => {
    isShowingFriendIdeaForm = false;
    $friendIdeas.refetch();
  };

  const getIdeaTypeHeading = (ideaType: string): string => {
    const headings: { [key in FriendIdeaTypeEnum]: string } = {
      activity: 'Activities',
      gift: 'Gifts',
      conversation: 'Conversations',
      other: 'Other',
      place: 'Places',
    };
    return headings[ideaType as FriendIdeaTypeEnum] || 'Unknown';
  };

  const handleToggleAddIdeaForm = () => {
    isShowingFriendIdeaForm = !isShowingFriendIdeaForm;
  };
</script>

<div class="wrapper">
  <div>
    <a class="button" href="/friends"> Back </a>
  </div>
  {#if !friendId || $friend.isLoading}
    <span>Loading...</span>
  {/if}
  {#if $friend.error}
    <span>Error: {$friend.error.message}</span>
  {/if}
  {#if $friend.isSuccess}
    <h1>{$friend.data.first_name} {$friend.data.last_name}</h1>

    {#each attributes as attribute}
      <div class="attribute">
        <b>{attribute.label}:</b>

        {#if attribute.value != null}
          {attribute.format?.(attribute.value) ?? attribute.value}
        {:else}
          Not set
        {/if}
      </div>
    {/each}

    <br />
    <a href="/friends/{friendId}/edit">Edit</a>

    <br /><br />

    <h2>Ideas</h2>

    {#if isShowingFriendIdeaForm}
      <FriendIdeaForm {friendId} onSuccess={handleAddedFriendIdea} />
      <br />
      <button on:click={handleToggleAddIdeaForm}>Cancel</button>
      <br />
    {:else}
      <button on:click={handleToggleAddIdeaForm}> Add Idea </button>
    {/if}

    {#if $friendIdeas.isLoading}
      <span>Loading...</span>
    {/if}
    {#if $friendIdeas.error}
      <span>Error: {$friendIdeas.error.message}</span>
    {/if}
    {#if $friendIdeas.isSuccess}
      {#if !friendIdeaGroups}
        <span>No ideas yet</span>
      {:else}
        {#each Object.entries(friendIdeaGroups) as [type, idea], i}
          <div class="idea__group">
            <h3>{getIdeaTypeHeading(type)}</h3>
            {#each idea as idea, j}
              <div class="idea__item">{idea.content}</div>
            {/each}
          </div>
        {/each}
      {/if}
    {/if}

    <br /><br />

    <div>{$friend.isFetching || $friendIdeas.isFetching ? 'Refreshing data...' : ' '}</div>
  {/if}
</div>

<style>
  .wrapper {
    padding: 40px;
  }

  .attribute {
    margin-bottom: 8px;
  }

  a {
    text-decoration: underline;
  }

  h2 {
    font-size: 1.5rem;
  }

  h3 {
    font-size: 1.2rem;
  }

  .idea__group {
    margin-bottom: 32px;
  }

  .idea__item {
    margin: 8px 0;
  }
</style>
