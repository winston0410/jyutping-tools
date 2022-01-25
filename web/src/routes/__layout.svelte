<script lang="ts" context="module">
	import 'modern-css-reset';
	import '@fontsource/karla';
	import Notifications from 'svelte-notifications';
	import Toast from '$lib/Toast.svelte';
    import SkipToContent from "carbon-components-svelte/src/UIShell/SkipToContent.svelte"
    import Content from "carbon-components-svelte/src/UIShell/Content.svelte"
</script>

<script lang="ts">
</script>

<SkipToContent />
<Content>
    <Notifications item={Toast}>
        <slot />
    </Notifications>
</Content>
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
