<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
	//  import storage from './debug'
	import type { Obj, Errors, Touched } from '@felte/common';
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let ref = null;
	export let initValue: string;
	export let errors: Errors<Obj>;
	export let warnings: Errors<Obj>;
	export let touched: Touched<Obj>;
	export let placeholder: string;
	export let spellcheck = true;

	$: isInvalid = errors[name] || warnings[name];
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
				class:error-border={errors[name]}
				class:warning-border={!errors[name] && warnings[name]}
				class:ok-border={!errors[name] && !warnings[name]}
				bind:this={ref}
				use:storage={{ name, initValue }}
				{name}
				{placeholder}
				{spellcheck}
				on:input
				on:change
			/>
		</label>

		{#if isInvalid}
			{#if !errors[name] && warnings[name]}
				<slot name="warning" />
			{:else}
				<slot name="error" />
			{/if}
		{/if}
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
				use:storage={{ name }}
				{name}
				on:input
				on:change
			/>
		</label>
		{#if isInvalid}
			{#if !errors[name] && warnings[name]}
				<slot name="warning" />
			{:else}
				<slot name="error" />
			{/if}
		{/if}
	</div>
{/if}

<style lang="scss">
	.input {
		@include textarea;
	}

	.input:focus {
		outline: $outline-width dashed var(--color-text);
	}

	.error-border:focus {
		outline-color: $error;
	}
	.warning-border:focus {
		outline-color: $warning;
	}
	.ok-border:focus {
		outline-color: $ok;
	}

	.input::placeholder {
		@include placeholder-text;
	}

	.number-input {
		@include textarea;
		height: initial;
	}

	.label {
		@include input-header-layout;
		font-weight: 700;
	}
</style>
