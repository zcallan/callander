import { createQuery } from '@tanstack/svelte-query';
import { getFriends } from '../api/friends';
import type { Friend } from '../types/generated';

export const friendsQuery = (params: any) => {
  console.log('friendsQuery', params);

  createQuery<Friend[], Error>({
    queryKey: ['friends'],
    queryFn: ({ meta }) => getFriends({ limit: meta?.limit }),
  });
};
