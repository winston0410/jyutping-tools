<script lang="ts" context="module">
	import type { Toast } from '$lib/toast';
	import { ToastKind } from '$lib/toast';
	import { fade } from 'svelte/transition';
	//  @ts-ignore
	import Close from 'virtual:icons/carbon/close';
</script>

<script lang="ts">
	// NOTE notification and onRemove are required props for svelte-notifications
	// NOTE notification are the props that will be passed to the component by svelte-notifications
	export let notification: Toast;
	export let onRemove = null;
	// NOTE withoutStyle prop will be passed in, but will not be used
	export let withoutStyles = false;

	const { kind, text } = notification;

	const handleClick = () => {
		onRemove();
	};
</script>

<div
	class="toast"
	class:error={kind === ToastKind.Error}
	class:warning={kind === ToastKind.Warning}
	class:info={kind === ToastKind.Info}
	class:ok={kind === ToastKind.Ok}
	role="status"
	aria-live="polite"
	in:fade
	out:fade
>
	<div class="toast-inner">
		<span class="jam">
			{text}
		</span>
		<!--  <button class="close-button" type="button" on:click={handleClick}>  -->
		<!--  <Close />  -->
		<!--  </button>  -->
	</div>
</div>

<style lang="scss">
	$radius: 8px;

	.toast {
		border-radius: $radius;
		color: $white;
		margin-bottom: 12px;
	}

	.jam {
		padding: 12px;
	}

	.toast-inner {
		display: flex;
		flex-wrap: wrap;
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

	.close-button {
		display: flex;
		justify-content: center;
		align-items: center;
		background: $white;
		padding: 12px;
		border-top-right-radius: $radius;
		border-bottom-right-radius: $radius;
		transition: background 500ms;
	}

	.close-button:hover {
		background: darken($white, 10%);
	}
</style>
