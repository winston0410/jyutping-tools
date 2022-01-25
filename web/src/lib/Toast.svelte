<script lang="ts" context="module">
	import type { Toast } from '$lib/toast';
	import { ToastKind } from '$lib/toast';
	import { fade } from 'svelte/transition';
</script>

<script lang="ts">
	// NOTE notification and onRemove are required props for svelte-notifications
	// NOTE notification are the props that will be passed to the component by svelte-notifications
	export let notification: Toast;
	export let onRemove = null;
	// NOTE withoutStyle prop will be passed in, but will not be used
	export const withoutStyles = false;

	const handleClick = () => {
		onRemove();
	};
</script>

<div
	class="toast"
	class:error={notification.kind === ToastKind.Error}
	class:warning={notification.kind === ToastKind.Warning}
	class:info={notification.kind === ToastKind.Info}
	class:ok={notification.kind === ToastKind.Ok}
	role="status"
	aria-live="polite"
	in:fade
	out:fade
>
	<span>
		{notification.text}
	</span>

	<button type="button" on:click={handleClick}>
        Close
    </button>
</div>

<style lang="scss">
	.toast {
		padding: 12px;
		border-radius: 5px;
		color: $white;
	}

	.error {
		background-color: $error;
	}

	.warning {
		background-color: $warning;
	}

	.ok {
		background-color: $ok;
	}

	.info {
        background-color: $info;
	}
</style>
