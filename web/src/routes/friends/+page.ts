import { getFriends } from '$lib/api/friends';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
  const { queryClient } = await parent();

  await queryClient.prefetchQuery({
    queryKey: ['friends'],
    queryFn: getFriends,
  });
};
