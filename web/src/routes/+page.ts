import { getFriends } from '$lib/api/friends';
import { getPosts } from '$lib/api/posts';
import type { PostsFindAllQuery } from '../../../api/bindings/PostsFindAllQuery';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
  const { queryClient } = await parent();

  const postsOptions: PostsFindAllQuery = { page: 1, per_page: 10 };

  await Promise.allSettled([
    queryClient.prefetchQuery({
      queryKey: ['friends'],
      queryFn: getFriends,
    }),
    queryClient.prefetchQuery({
      queryKey: ['posts', postsOptions],
      queryFn: () => getPosts(postsOptions),
    }),
  ]);
};
