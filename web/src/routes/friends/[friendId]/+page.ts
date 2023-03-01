import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	const friendId: string = params.friendId;
	return { friendId };
};
