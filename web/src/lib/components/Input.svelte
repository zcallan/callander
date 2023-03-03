<script lang="ts">
  import { DateInput } from 'date-picker-svelte';
  import dayjs from 'dayjs';

  type SelectOption = { label: string; value: string };

  export let name: string;
  export let type: string = 'text';

  export let label: string | undefined = undefined;
  export let value: string | undefined = undefined;
  export let date: Date | undefined = undefined;
  export let error: string | undefined = undefined;
  export let minYear: number = 1950;
  export let maxYear: number | undefined = undefined;
  export let dateFormat: string = 'yyyy-MM-dd';
  export let placeholder: string | undefined = undefined;
  export let options: string | SelectOption[] | undefined = undefined;
  export let required: boolean | undefined = undefined;

  function typeAction(node: HTMLInputElement) {
    node.type = type;
  }

  const _options: SelectOption[] =
    typeof options === 'string'
      ? options.split(',').map((option) => {
          const [value, label] = option.split(':');
          return { label, value };
        })
      : options || [];
</script>

<div class="input">
  {#if label}<label for={name}>{label}</label>{/if}

  {#if $$slots.default}
    <slot />
  {:else if type === 'date'}
    <DateInput
      bind:value={date}
      format={dateFormat}
      min={dayjs().year(minYear).toDate()}
      max={maxYear ? dayjs().year(maxYear).toDate() : undefined}
      placeholder={placeholder ?? 'YYYY-MM-DD'}
      closeOnSelection
    />
  {:else if type === 'select'}
    <select bind:value {name} {required} {...$$restProps}>
      {#if placeholder}<option value="">{placeholder}</option>{/if}
      {#each _options as option}
        <option value={option.value}>{option.label ?? option.value}</option>
      {/each}
    </select>
  {:else}
    <input use:typeAction {name} bind:value {placeholder} {required} {...$$restProps} />
  {/if}

  {#if error}<span class="error">{error}</span>{/if}
</div>

<style>
  .input {
  }

  label {
    margin-bottom: 4px;
    display: block;
  }

  input {
    display: block;
    width: 100%;
  }

  :global(.date-time-field input) {
    display: block;
    width: 100% !important;
  }

  select {
    width: 100%;
    height: 28px;
    font-size: 16px;
  }

  .error {
  }
</style>
