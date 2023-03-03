import { createMutation, type CreateMutationOptions } from '@tanstack/svelte-query';
import type { Friend, UpdateFriend } from '$lib/types/generated';
import { updateFriend } from '$lib/api/friends';

type UpdateFriendMutationOption = Omit<
  CreateMutationOptions<Friend, Error, UpdateFriend, unknown>,
  'mutationFn'
>;

export const updateFriendMutation = (id: string, options?: UpdateFriendMutationOption) => {
  return createMutation<Friend, Error, UpdateFriend>((values) => updateFriend(id, values), options);
};
