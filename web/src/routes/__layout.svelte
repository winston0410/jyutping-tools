<script lang="ts" context="module">
	import 'modern-css-reset';
	import '@fontsource/karla';
	import Notifications from 'svelte-notifications';
	import Toast from '$lib/Toast.svelte';
</script>

<script lang="ts">
</script>

<Notifications item={Toast}>
    <main id="main-content">
        <slot />
    </main>
</Notifications>
<!--  <footer>  -->
<!--  <p>Hello world</p>  -->

<!--  </footer>  -->
<style lang="scss">
	:global(html) {
		font-size: 16px;
	}
	:global(body) {
		font-family: 'Karla';
		color: $black;
	}

	:global(body) {
		background: hsla(46, 75%, 87%, 1);
	}

	:global(button) {
		appearance: none;
		margin: 0;
		padding: 0;
		border: 0;
		cursor: pointer;
	}

	footer {
		background: $white;
		padding: 12px;
	}
</style>
