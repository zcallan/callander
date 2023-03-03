<script lang="ts">
  import dayjs from 'dayjs';
  import type { Friend, MetAtAccuracyEnum, UpdateFriend } from '$lib/types/generated';
  import { createMutation } from '@tanstack/svelte-query';
  import { updateFriend } from '$lib/api/friends';
  import { serializeForm } from '$lib/utils/serializeForm';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import type { PageData } from './$types';
  import { getFriendByIdQuery } from '$lib/queries/friends';
  import { updateFriendMutation } from '$lib/mutations/friends';
  import Input from '$lib/components/Input.svelte';
  import range from '$lib/utils/range';

  export let data: PageData;

  const DATE_FORMAT = 'YYYY-MM-DD';
  const BIRTH_YEAR = 1997;
  const CURRENT_AGE = dayjs().diff(dayjs().year(BIRTH_YEAR), 'years');
  const MONTH_OPTIONS =
    '0:January,1:February,2:March,3:April,4:May,5:June,6:July,7:August,8:September,9:October,10:November,11:December';
  const YEAR_OPTIONS = range(CURRENT_AGE + 1, BIRTH_YEAR)
    .reverse()
    .join(',');

  const friendId: string = data.friendId;

  const friendQuery = getFriendByIdQuery(friendId);

  let dateOfBirth: Date;
  let metAtYear: string;
  let metAtMonth: string;
  let metAtDate: Date;
  let metAtAccuracy: MetAtAccuracyEnum;

  friendQuery.subscribe((friend) => {
    if (friend.data?.date_of_birth) dateOfBirth = new Date(friend.data.date_of_birth);
    if (friend.data?.met_at_accuracy) metAtAccuracy = friend.data.met_at_accuracy;

    if (friend.data?.met_at) {
      const metAt = dayjs(friend.data.met_at);
      metAtDate = metAt.toDate();
      metAtYear = metAt.year().toString();
      metAtMonth = metAt.month().toString();
    }
  });

  const updateMutation = updateFriendMutation(friendId, {
    onSuccess: (updatedFriend: Friend) => {
      goto(`/friends/${updatedFriend.id}`);
    },
  });

  function handleSubmitCreateFriend(event: any) {
    const data = serializeForm(event.target);

    const _dateOfBirth = dateOfBirth ? dayjs(dateOfBirth).format(DATE_FORMAT) : undefined;

    let _metAt: string | undefined = undefined;
    if (metAtAccuracy === 'year') {
      _metAt = dayjs().startOf('year').year(parseInt(metAtYear)).format(DATE_FORMAT);
    } else if (metAtAccuracy === 'month') {
      _metAt = dayjs()
        .startOf('year')
        .year(parseInt(metAtYear))
        .month(parseInt(metAtMonth))
        .format(DATE_FORMAT);
    } else if (metAtAccuracy === 'day') {
      _metAt = dayjs(metAtDate).format(DATE_FORMAT);
    }

    const updatedFriend: UpdateFriend = {
      first_name: data.first_name,
      last_name: data.last_name,
      date_of_birth: _dateOfBirth,
      met_at: _metAt,
      met_at_accuracy: metAtAccuracy || undefined,
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

      <Input label="Date of Birth" type="date" name="date_of_birth" bind:date={dateOfBirth} />

      <Input
        label="Friends since"
        name="met_at_accuracy"
        type="select"
        options=":--,year:Year,month:Month,day:Day"
        bind:value={metAtAccuracy}
      />

      {#if metAtAccuracy === 'year'}
        <Input
          label="Year"
          type="select"
          name="met_at_year"
          options={YEAR_OPTIONS}
          bind:value={metAtYear}
        />
      {:else if metAtAccuracy === 'month'}
        <Input
          label="Month"
          type="select"
          name="met_at_month"
          options={MONTH_OPTIONS}
          bind:value={metAtMonth}
        />
        <Input
          label="Year"
          type="select"
          name="met_at_year"
          options={YEAR_OPTIONS}
          bind:value={metAtYear}
        />
      {:else if metAtAccuracy === 'day'}
        <Input label="Date" type="date" name="met_at_day" bind:date={metAtDate} />
      {/if}

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
