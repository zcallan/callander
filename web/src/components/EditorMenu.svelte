<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import EditorMenuButton from './EditorMenuButton.svelte';

	export let editor: Editor;

	const chainer = (editor: any, fn: any): any => editor.chain().focus()[fn]().run();

	const toggleBold = (editor: Editor) => chainer(editor, 'toggleBold');
	const toggleItalic = (editor: Editor) => editor.chain().focus().toggleItalic().run();
	const toggleStrike = (editor: Editor) => editor.chain().focus().toggleStrike().run();
</script>

{#if editor}
	<div class="fixed-menu">
		<EditorMenuButton {editor} key="bold" on:click={() => toggleBold(editor)} />
		<EditorMenuButton {editor} key="italic" on:click={() => toggleItalic(editor)} />
		<EditorMenuButton {editor} key="strike" on:click={() => toggleStrike(editor)} />
	</div>
{/if}

<style>
	.fixed-menu {
		background: #eee;
		padding: 5px;
	}
</style>
