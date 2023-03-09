import { createQuery } from '@tanstack/svelte-query';
import { getFriendById, getFriendIdeaById, getFriendIdeas } from '../api/friends';
import type { Friend, FriendsIdea } from '$lib/types/generated';
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

export const getFriendIdeasQuery = (friendId: string) => {
  return createQuery<FriendsIdea[], Error>({
    queryKey: ['friend-ideas', friendId],
    queryFn: () => getFriendIdeas(friendId),
  });
};

export const getFriendIdeaByIdQuery = (id: string) => {
  return createQuery<FriendsIdea, Error>({
    queryKey: ['friend-idea', id],
    queryFn: () => getFriendIdeaById(id),
  });
};
