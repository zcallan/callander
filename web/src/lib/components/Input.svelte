<script lang="ts">
  import { DateInput } from 'date-picker-svelte';
  import dayjs from 'dayjs';

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

  function typeAction(node: HTMLInputElement) {
    node.type = type;
  }
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
    />
  {:else}
    <input use:typeAction {name} bind:value {placeholder} {...$$restProps} />
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

  .error {
  }
</style>
