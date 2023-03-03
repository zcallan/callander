<script lang="ts">
  import { createQuery } from '@tanstack/svelte-query';
  import type { PageData } from './$types';
  import { getFriendById } from '$lib/api/friends';
  import type { Friend } from '$lib/types/generated';
  import dayjs from 'dayjs';

  export let data: PageData;

  const friendId: string = data.friendId;

  const friend = createQuery<Friend, Error>({
    queryKey: ['friend', friendId],
    queryFn: () => getFriendById(friendId),
  });

  let attributes: any[] = [];

  $: attributes = [
    {
      label: 'Date of birth',
      value: $friend.data?.date_of_birth,
      format: (v: any) => v && dayjs(v).format('D MMM YYYY'),
    },
    {
      label: 'Years old',
      value: $friend.data?.date_of_birth,
      format: (value: any) => {
        if (!value) return null;
        const dateOfBirth = dayjs(value);
        const currentYear = dayjs().year();
        const daysUntilBday = dateOfBirth.year(currentYear).isAfter(dayjs())
          ? dateOfBirth.year(currentYear).diff(dayjs(), 'days')
          : dateOfBirth.year(currentYear + 1).diff(dayjs(), 'days');
        const yearsOld = dayjs().diff(dateOfBirth, 'years');
        return `${yearsOld} (${daysUntilBday}d until bday)`;
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
    <div>{$friend.isFetching ? 'Refreshing data...' : ' '}</div>
  {/if}
</div>

<style>
  .wrapper {
    text-align: center;
    padding: 40px;
  }

  .attribute {
    margin-bottom: 8px;
  }
</style>
