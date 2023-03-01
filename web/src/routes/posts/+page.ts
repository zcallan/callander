import { getPosts } from '../../api/posts';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { queryClient } = await parent();

	await queryClient.prefetchQuery({
		queryKey: ['posts'],
		queryFn: getPosts
	});
};
