import type { Paginated, Post, NewPost, PostsFindAllQuery } from '$lib/types/generated';
import apiCall from '$lib/utils/apiCall';

export const getPosts = async (params: PostsFindAllQuery) => {
  const urlParams = new URLSearchParams({
    page: String(params.page),
    per_page: String(params?.per_page ?? 10),
  });

  return apiCall<Paginated<Post[]>>(`/posts?${urlParams}`);
};

export const getPostById = async (id: string) => {
  return apiCall<Post>(`/posts/${id}`);
};

export const createPost = async (post: NewPost) => {
  return apiCall<Post>('/posts', {
    method: 'POST',
    body: JSON.stringify(post),
  });
};
