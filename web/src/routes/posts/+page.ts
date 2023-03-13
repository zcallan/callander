import { getPosts } from '$lib/api/posts';
import type { PostsFindAllQuery } from '$lib/types/generated';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
  const { queryClient } = await parent();

  const options: PostsFindAllQuery = { page: 1, per_page: 10 };

  await queryClient.prefetchQuery({
    queryKey: ['posts', options],
    queryFn: () => getPosts(options),
  });
};
