<script lang="ts">
	import type { Editor } from '@tiptap/core';

	export let editor: Editor;
	export let key: string;
</script>

{#if editor}
	<button type="button" class:active={editor.isActive(key)} on:click>
		{key}
	</button>
{/if}

<style>
	button {
		margin: 0;
	}

	button.active {
		background: black;
		color: white;
	}
</style>
