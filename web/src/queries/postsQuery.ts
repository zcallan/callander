import { createQuery } from '@tanstack/svelte-query';
import { getPosts } from '../api/posts';
import type { Post } from '../types/generated';

const postsQuery = (params: any) => {
  return createQuery<Post[], Error>({
    queryKey: ['posts', params?.limit],
    queryFn: () => getPosts({ limit: params?.limit }),
  });
};

export default postsQuery;
