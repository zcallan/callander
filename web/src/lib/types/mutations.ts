import type { CreateMutationOptions } from '@tanstack/svelte-query';

export type MutationOptions<ReturnType, FormValues, E = Error> = Omit<
  CreateMutationOptions<ReturnType, E, FormValues, unknown>,
  'mutationFn'
>;
