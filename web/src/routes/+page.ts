import { getFriends } from '../api/friends';
import { getPosts } from '../api/posts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { queryClient } = await parent();

	await Promise.allSettled([
		queryClient.prefetchQuery({
			queryKey: ['friends'],
			queryFn: getFriends
		}),
		queryClient.prefetchQuery({
			queryKey: ['posts'],
			queryFn: getPosts
		})
	]);
};
