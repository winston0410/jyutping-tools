<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
	import type { Obj, Errors, Touched } from '@felte/common';
	import ErrorMessage from './ErrorMessage.svelte';
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let errors: Errors<Obj>;
	export let warnings: Errors<Obj>;
	export let touched: Touched<Obj>;
	export let placeholder: string;
	export let spellcheck = true;
</script>

{#if type === 'textarea'}
	<div>
		<label for={name}>
			<span class="label">
				<slot />
			</span>
			<textarea
				id={name}
				class="input"
				class:error-border={touched[name] && errors[name]}
				class:warning-border={touched[name] && !errors[name] && warnings[name]}
				class:ok-border={touched[name] && !errors[name] && !warnings[name]}
				use:storage={name}
				{name}
				{placeholder}
				{spellcheck}
				on:input
				on:change
			/>
		</label>
		<ErrorMessage {errors} {warnings} {name} />
	</div>
{:else if type === 'number'}
	<div>
		<label for={name}>
			<span class="label">
				<slot />
			</span>
			<input
				id={name}
				class="number-input"
				class:error-border={touched[name] && errors[name]}
				class:warning-border={touched[name] && !errors[name] && warnings[name]}
				class:ok-border={touched[name] && !errors[name] && !warnings[name]}
				use:storage={name}
				{name}
				on:input
				on:change
			/>
		</label>
		<ErrorMessage {errors} {warnings} {name} />
	</div>
{/if}

<style lang="scss">
	.input {
		@include textarea;
	}

	.input::placeholder {
		color: $grey;
	}

	.number-input {
		@include textarea;
		height: initial;
	}

	.label {
		font-weight: 700;
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
</style>
