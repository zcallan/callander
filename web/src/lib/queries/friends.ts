import { createQuery } from '@tanstack/svelte-query';
import { getFriendById } from '../api/friends';
import type { Friend } from '$lib/types/generated';
import { getFriends } from '../api/friends';

// TODO: Paginated<Friend[]>
export const getFriendsQuery = (params: any) => {
  return createQuery<Friend[], Error>({
    queryKey: ['friends', params.page],
    queryFn: () => getFriends({ page: params.page }),
  });
};

export const getFriendByIdQuery = (id: string) => {
  return createQuery<Friend, Error>({
    queryKey: ['friend', id],
    queryFn: () => getFriendById(id),
  });
};
