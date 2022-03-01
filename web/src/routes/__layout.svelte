<script lang="ts" context="module">
	import 'modern-css-reset';
	import '@fontsource/karla';
	import Notifications from 'svelte-notifications';
	import Toast from '$lib/Toast.svelte';
</script>

<script lang="ts">
</script>

<header class="header">
	<span class="logo">Cantonese.tool</span>
</header>
<Notifications item={Toast}>
	<main id="main-content" class="shell">
		<slot />
	</main>
</Notifications>

<style lang="scss">
	:global(html) {
		font-size: 20px;
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

	.shell {
		padding: 20px;
		margin-bottom: 3rem;
	}

	.header {
		min-height: $header-height;
		padding: 12px;
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
