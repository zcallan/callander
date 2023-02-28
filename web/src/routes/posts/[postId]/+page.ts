import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const postId: string = params.postId;
	return { postId };
};
