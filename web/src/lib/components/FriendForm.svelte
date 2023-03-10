<script lang="ts">
  import dayjs from 'dayjs';
  import type { Friend, MetAtAccuracyEnum, NewFriend, UpdateFriend } from '$lib/types/generated';
  import Input from '$lib/components/Input.svelte';
  import range from '$lib/utils/range';
  import { onMount } from 'svelte';

  export let onSubmit: (values: NewFriend | UpdateFriend) => void;
  export let isEditing: boolean = false;
  export let isLoading: boolean = false;
  export let errorMessage: string | null = null;
  export let defaultValues: Partial<Friend> | undefined = undefined;

  const DATE_FORMAT = 'YYYY-MM-DD';
  const BIRTH_YEAR = 1997;
  const CURRENT_AGE = dayjs().diff(dayjs().year(BIRTH_YEAR), 'years');
  const MONTH_OPTIONS =
    '0:January,1:February,2:March,3:April,4:May,5:June,6:July,7:August,8:September,9:October,10:November,11:December';
  const YEAR_OPTIONS = range(CURRENT_AGE + 1, BIRTH_YEAR)
    .reverse()
    .join(',');

  let dateOfBirth: Date | undefined;
  let metAtYear: string | undefined;
  let metAtMonth: string | undefined;
  let metAtDate: Date | undefined;
  let metAtAccuracy: MetAtAccuracyEnum | undefined;
  let firstName: string = '';
  let lastName: string = '';

  onMount(() => {
    if (!defaultValues) return;

    firstName = defaultValues.first_name ?? '';
    lastName = defaultValues.last_name ?? '';
    metAtAccuracy = defaultValues.met_at_accuracy || undefined;

    if (defaultValues.date_of_birth) {
      dateOfBirth = new Date(defaultValues.date_of_birth);
    }

    if (defaultValues.met_at) {
      const metAt = dayjs(defaultValues.met_at);
      metAtDate = metAt.toDate();
      metAtYear = metAt.year().toString();
      metAtMonth = metAt.month().toString();
    }
  });

  function handleSubmitCreateFriend() {
    const _dateOfBirth = dateOfBirth ? dayjs(dateOfBirth).format(DATE_FORMAT) : undefined;

    let _metAt: string | undefined = undefined;
    if (metAtAccuracy === 'year' && metAtYear) {
      _metAt = dayjs().startOf('year').year(parseInt(metAtYear)).format(DATE_FORMAT);
    } else if (metAtAccuracy === 'month' && metAtYear && metAtMonth) {
      _metAt = dayjs()
        .startOf('year')
        .year(parseInt(metAtYear))
        .month(parseInt(metAtMonth))
        .format(DATE_FORMAT);
    } else if (metAtAccuracy === 'day') {
      _metAt = dayjs(metAtDate).format(DATE_FORMAT);
    }

    onSubmit?.({
      first_name: firstName,
      last_name: lastName,
      date_of_birth: _dateOfBirth || undefined,
      met_at: _metAt || undefined,
      met_at_accuracy: metAtAccuracy || undefined,
    });
  }
</script>

<form on:submit|preventDefault={handleSubmitCreateFriend}>
  <Input label="First Name" name="first_name" required bind:value={firstName} />

  <Input label="Last Name" name="last_name" required bind:value={lastName} />

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

  {#if errorMessage}
    <p>Error: {errorMessage}</p>
  {/if}

  <button type="submit" disabled={isLoading}>
    {isLoading ? 'Loading...' : isEditing ? 'Update' : 'Create'}
  </button>
</form>

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
</style>
