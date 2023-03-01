<script lang="ts">
	import Friend from '../../../components/Friend.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const friendId: string = data.friendId;
</script>

<Friend {friendId} />
