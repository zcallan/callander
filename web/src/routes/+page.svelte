<script lang="ts">
  import { getPostsQuery } from '$lib/queries/posts';
  import { getFriendsQuery } from '$lib/queries/friends';
  import dayjs from 'dayjs';
  import type { Friend } from '../../../api/bindings/Friend';

  const CURRENT_YEAR = new Date().getFullYear();
  const BDAY_THRESHOLD = 60;

  // This data is cached by prefetchQuery in +page.ts so no fetch actually happens here
  const friendsQuery = getFriendsQuery({ page: 1, per_page: 10 });

  const formatBday = (date: string) => {
    const formattedDate = dayjs(date).format('MMM D');
    const daysUntilBday = dayjs(date).set('year', CURRENT_YEAR).diff(dayjs(), 'days');
    return `${formattedDate} (${Math.abs(daysUntilBday)} days ${
      daysUntilBday > 0 ? 'away' : 'ago'
    })`;
  };

  let upcomingBdays: Friend[] = [];
  let recentBdays: Friend[] = [];
  $: {
    if ($friendsQuery.isSuccess) {
      upcomingBdays = $friendsQuery.data.filter((friend) => {
        const bday = dayjs(friend.date_of_birth).set('year', CURRENT_YEAR);
        const diff = bday.diff(dayjs(), 'days');
        return diff >= 0 && diff <= BDAY_THRESHOLD;
      });
      recentBdays = $friendsQuery.data.filter((friend) => {
        const bday = dayjs(friend.date_of_birth).set('year', CURRENT_YEAR);
        const diff = bday.diff(dayjs(), 'days');
        return diff < 0 && diff > -BDAY_THRESHOLD;
      });
    }
  }
</script>

<svelte:head>
  <title>Callander</title>
  <meta name="description" content="Lorem ipsum" />
</svelte:head>

<section>
  <h1>Welcome.</h1>

  <a href="/friends">
    <h2>Birthdays</h2>
  </a>

  {#if $friendsQuery.isLoading}
    <p>Loading...</p>
  {:else if $friendsQuery.isError}
    <p>Error: {$friendsQuery.error.message}</p>
  {:else if $friendsQuery.isSuccess}
    {#if !$friendsQuery.data?.length}
      <p>No friends, loser</p>
    {/if}

    {#if upcomingBdays.length}
      <h3>Upcoming</h3>
      {#each upcomingBdays as friend}
        <a href="/friends/{friend.id}">
          {friend.first_name}
          {friend.last_name}
          - {formatBday(friend.date_of_birth)}
        </a>
      {/each}
      <br />
    {/if}

    {#if recentBdays.length}
      <h3>Recent</h3>
      {#each recentBdays as friend}
        <a href="/friends/{friend.id}">
          {friend.first_name}
          {friend.last_name}
          - {formatBday(friend.date_of_birth)}
        </a>
      {/each}
      <br />
    {/if}
  {/if}

  <br />
  <hr />
  <br />

  <a href="/friends/new">Create Friend</a>
  <a href="/posts/new">Create Post</a>
</section>

<style>
  label {
    display: block;
  }

  a {
    display: block;
    padding: 5px 0;
  }

  section {
    padding: 24px 32px;
  }
</style>
