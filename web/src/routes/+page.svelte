<script lang="ts">
  import type { Friend } from '../../../api/bindings/Friend';
  import dayjs from 'dayjs';
  import { getFriendsQuery } from '$lib/queries/friends';
  import formatBirthdayString from '$lib/utils/formatBirthdayString';

  const CURRENT_YEAR = new Date().getFullYear();
  const BDAY_THRESHOLD_DAYS = 60;

  const friendsQuery = getFriendsQuery({ page: 1, per_page: 10 });

  let upcomingBdays: Friend[] = [];
  let recentBdays: Friend[] = [];
  $: {
    if ($friendsQuery.isSuccess) {
      let _upcomingBdays: Friend[] = [];
      let _recentBdays: Friend[] = [];

      $friendsQuery.data.forEach((friend) => {
        const bday = dayjs(friend.date_of_birth).set('year', CURRENT_YEAR);
        const diff = bday.diff(dayjs(), 'days');

        if (diff >= 0 && diff <= BDAY_THRESHOLD_DAYS) {
          _upcomingBdays.push(friend);
        } else if (diff < 0 && diff > -BDAY_THRESHOLD_DAYS) {
          _recentBdays.push(friend);
        }
      });

      upcomingBdays = _upcomingBdays;
      recentBdays = _recentBdays;
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
          - {formatBirthdayString(friend.date_of_birth)}
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
          - {formatBirthdayString(friend.date_of_birth)}
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
  a {
    display: block;
    padding: 5px 0;
  }

  section {
    padding: 24px 32px;
  }
</style>
