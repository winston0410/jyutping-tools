<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
	import type { Obj, Errors, Touched } from '@felte/common';
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let error: Errors<Obj>;
	export let touched: Touched<Obj>;

        
</script>

{#if type === 'textarea'}
	<label class="textarea">
		<span class="label">
			<slot />
		</span>
		{#if error[name]}
			<span>
				{error[name][0]}
			</span>
		{/if}
		<textarea
			class="input"
			class:invalid={touched[name] && error[name]}
			class:valid={touched[name] && !error[name]}
			use:storage={name}
			{name}
			on:input
			on:change
		/>
	</label>
{:else}{/if}

<style lang="scss">
	@mixin input-field {
		display: block;
		border: 2px solid $black;
		background: $white;
		outline: none;
	}

	.input {
		@include input-field;
		width: 100%;
		resize: none;
		padding: 12px;
		height: 225px;
	}

	.invalid {
		border: 2px solid $error;
	}

	.valid {
		border: 2px solid $ok;
	}
</style>
