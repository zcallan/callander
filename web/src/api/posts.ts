import type { NewPost } from '../types/generated';

export const getPosts = async () => {
	const res = await fetch('http://localhost:8080/posts');
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
			'Content-Type': 'application/json'
		}
	});
	const data = await res.json();

	return data;
};
