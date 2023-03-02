import { createQuery } from '@tanstack/svelte-query';
import { getPosts } from '../api/posts';
import type { Paginated, Post, PostsFindAllQuery } from '$lib/types/generated';

const postsQuery = (params: PostsFindAllQuery) => {
  return createQuery<Paginated<Post[]>, Error>({
    queryKey: ['posts', params.page],
    queryFn: () => getPosts({ page: params.page }),
  });
};

export default postsQuery;
