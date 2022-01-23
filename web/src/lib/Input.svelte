<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
	import type { Obj, Errors, Touched } from '@felte/common';
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let error: Errors<Obj>;
	export let touched: Touched<Obj>;
	export let placeholder: string = '';
</script>

{#if type === 'textarea'}
	<label class="textarea" for={name}>
		<span class="label">
			<slot />
		</span>
		<textarea
            id={name}
            {placeholder}
			class="input"
			class:invalid={touched[name] && error[name]}
			class:valid={touched[name] && !error[name]}
			use:storage={name}
			{name}
            {...$$restProps}
			on:input
			on:change
		/>
	</label>
	{#if error[name]}
		<span class="error-message">
            {error[name][0]}
		</span>
	{/if}
{:else}{/if}

<style lang="scss">
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

	.invalid {
		border: 2px solid $error;
	}

	.valid {
		border: 2px solid $ok;
	}

    .error-message{
        color: $error;
    }
</style>
