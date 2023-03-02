import type { NewPost, PostsFindAllQuery } from '$lib/types/generated';

export const getPosts = async (params: PostsFindAllQuery) => {
  const urlParams = new URLSearchParams({
    page: String(params.page),
    per_page: String(params?.per_page ?? 10),
  });

  const res = await fetch(`http://localhost:8080/posts?${urlParams}`);
  const data = await res.json();

  return data;
};

export const getPostById = async (id: string) => {
  const res = await fetch(`http://localhost:8080/posts/${id}`);
  const data = await res.json();

  return data;
};

export const createPost = async (post: NewPost) => {
  const res = await fetch('http://localhost:8080/posts', {
    method: 'POST',
    body: JSON.stringify(post),
    headers: {
      'Content-Type': 'application/json',
    },
  });
  const data = await res.json();

  return data;
};
