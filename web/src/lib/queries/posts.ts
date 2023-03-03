import { createQuery } from '@tanstack/svelte-query';
import { getPostById, getPosts } from '../api/posts';
import type { Paginated, Post, PostsFindAllQuery } from '$lib/types/generated';

export const getPostsQuery = (params: PostsFindAllQuery) => {
  return createQuery<Paginated<Post[]>, Error>({
    queryKey: ['posts', params.page],
    queryFn: () => getPosts({ page: params.page }),
  });
};

export const getPostByIdQuery = (id: string) => {
  return createQuery<Post, Error>({
    queryKey: ['post', id],
    queryFn: () => getPostById(id),
  });
};
