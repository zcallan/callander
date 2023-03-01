import type { NewPost } from '../types/generated';

type GetPostParams = {
  limit?: number;
};

export const getPosts = async (params?: any) => {
  const urlParams = new URLSearchParams({
    limit: params?.limit || 10,
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
