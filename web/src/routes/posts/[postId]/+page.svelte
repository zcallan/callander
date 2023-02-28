<script lang="ts">
	import Post from '../../../components/Post.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const postId: string = data.postId;
</script>

<Post {postId} />
