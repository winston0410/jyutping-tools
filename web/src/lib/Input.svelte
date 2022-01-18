<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
</script>

<script lang="ts">
	export let name: string;
    export let type: string;
    export let error: null | Array<string>;
</script>

{#if type === "textarea" }
<label class="textarea">
	<span class="textarea__label">
		<slot />
	</span>
    {#if error}
    <span>
        {error[0]}
    </span>
    {/if}
	<textarea class="textarea__input" use:storage={name} {name} on:input on:change />
</label>
{:else}
    
{/if}

<style lang="scss">
	@mixin input-field {
		display: block;
		border: 2px solid $black;
		background: $white;
	}

	.textarea__input {
		@include input-field;
		width: 100%;
		resize: none;
		padding: 12px;
		height: 225px;
	}
</style>
