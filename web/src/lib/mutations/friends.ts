import { createMutation } from '@tanstack/svelte-query';
import type {
  Friend,
  FriendsIdea,
  NewFriend,
  NewFriendsIdea,
  UpdateFriend,
  UpdateFriendsIdea,
} from '$lib/types/generated';
import { createFriend, createFriendIdea, updateFriend, updateFriendIdea } from '$lib/api/friends';
import type { MutationOptions } from '$lib/types/mutations';

export const createFriendMutation = (options?: MutationOptions<Friend, NewFriend>) => {
  return createMutation<Friend, Error, NewFriend>((values) => createFriend(values), options);
};

export const createFriendIdeaMutation = (
  options?: MutationOptions<FriendsIdea, NewFriendsIdea>
) => {
  return createMutation<FriendsIdea, Error, NewFriendsIdea>(
    (values) => createFriendIdea(values),
    options
  );
};

export const updateFriendMutation = (
  id: string,
  options?: MutationOptions<Friend, UpdateFriend>
) => {
  return createMutation<Friend, Error, UpdateFriend>((values) => updateFriend(id, values), options);
};

export const updateFriendIdeaMutation = (
  id: string,
  options?: MutationOptions<FriendsIdea, UpdateFriendsIdea>
) => {
  return createMutation<FriendsIdea, Error, UpdateFriendsIdea>(
    (values) => updateFriendIdea(id, values),
    options
  );
};
