<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
	import type { Obj, Errors, Touched } from '@felte/common';
    import ErrorMessage from './ErrorMessage.svelte'
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let errors: Errors<Obj>;
	export let warnings: Errors<Obj>;
	export let touched: Touched<Obj>;
	export let placeholder: string = '';
</script>

{#if type === 'textarea'}
	<div>
		<label class="textarea" for={name}>
			<span class="label">
				<slot />
			</span>
			<textarea
				id={name}
				{placeholder}
				class="input"
				class:error-border={touched[name] && errors[name]}
				class:warning-border={touched[name] && !errors[name] && warnings[name]}
				class:ok-border={touched[name] && !errors[name] && !warnings[name]}
				use:storage={name}
				{name}
				{...$$restProps}
				on:input
				on:change
			/>
		</label>
        <ErrorMessage {errors} {warnings} {name}/>
	</div>
{:else}
{/if}

<style lang="scss">
	$outline-width: 3px;

	@mixin input-field {
		display: block;
		border: 2px solid $black;
		background: $white;
		outline: none;
	}

	.label {
		font-weight: 700;
	}

	.input {
		@include input-field;
		width: 100%;
		resize: none;
		padding: 12px;
		height: 225px;
	}

	.input::placeholder {
		color: $grey;
	}

	.error-border {
		outline: $outline-width solid $error;
	}

	.warning-border {
		outline: $outline-width solid $warning;
	}

	.ok-border {
		outline: $outline-width solid $ok;
	}

	.error {
		color: $error;
	}

	.warning {
		color: $warning;
	}
</style>
