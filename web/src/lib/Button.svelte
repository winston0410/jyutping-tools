<!--  Good usage on translate: transformY  -->
<script lang="ts" context="module">
</script>

<script lang="ts">
	export let href: string | null = null;
	export let type: string = 'button';
	export let disabled: boolean = false;
	export let hover: boolean = true;
</script>

<!-- REF https://www.joshwcomeau.com/animation/3d-button/  -->


{#if href}
	<a {href} class="button" on:click>
		<span class="button-text" class:disabled class:hover>
			<slot />
		</span>
	</a>
{:else}
	<button {type} class="button" on:click {disabled}>
		<span class="button-text" class:disabled class:hover>
			<slot />
		</span>
	</button>
{/if}

<style lang="scss">
	.button {
		display: block;
		width: fit-content;
		color: #ffffff;
		border-radius: 5px;
		font-weight: 700;
		text-decoration: none;
		user-select: none;
		background-color: #000000;
		cursor: pointer;
		transition: all 0.5s;
	}

	.button-text {
		display: block;
		padding: 12px 26px;
		border-radius: 5px;
		background-color: var(--color-button);
		transform: translateY(-4px);
		min-width: 100px;
		text-align: center;
		transition: all 0.5s;
	}

	.hover:not(.disabled):hover {
		background-color: var(--color-button-highlight);
	}

	.disabled {
		background-color: var(--color-disabled);
		cursor: not-allowed;
	}
</style>
