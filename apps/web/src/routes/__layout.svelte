<script lang="ts" context="module">
	import 'modern-css-reset';
	import '@fontsource/karla';
	import Notifications from 'svelte-notifications';
	import Toast from '$lib/Toast.svelte';
</script>

<script lang="ts">
</script>

<header class="header">
    <a href="/" alt="To home" title="jyut.info">
        <span class="logo">jyut.info</span>
    </a>
</header>
<Notifications item={Toast}>
	<main id="main-content" class="shell">
		<slot />
	</main>
</Notifications>

<style lang="scss">
	:global(html) {
		font-size: 20px;
        scroll-behavior: smooth;
	}
	:global(body) {
		font-family: 'Karla';
		color: var(--color-text);
		background: var(--color-background);
	}

	:global(button) {
		appearance: none;
		margin: 0;
		padding: 0;
		border: 0;
		cursor: pointer;
	}

	:global(.link) {
		appearance: none;
		background: none;
		color: $blue;
		text-decoration: underline;
	}

	:global(.link:hover) {
		color: lighten($blue, 20%);
	}

	.shell {
		padding: 1rem;
		margin-bottom: 3rem;
	}

	.header {
		min-height: $header-height;
		padding: 1rem;
		display: flex;
		align-items: center;
		background: var(--color-header);
	}

	.logo {
		font-weight: 700;
	}

	footer {
		background: $white;
		padding: 12px;
	}
</style>
