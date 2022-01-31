<script lang="ts" context="module">
	import type { Obj, Errors } from '@felte/common';
</script>

<script lang="ts">
	export let errors: Errors<Obj>;
	export let warnings: Errors<Obj>;
	export let name: string;

	$: invalidity = errors[name] ? errors[name] : warnings[name];
</script>

{#if invalidity}
	<span class:error={errors[name]} class:warning={!errors[name] && warnings[name]}>
		{invalidity[0]}
	</span>
{/if}

<style lang="scss">
	.error {
		@include error-text;
	}

	.warning {
		@include warning-text;
	}
</style>
