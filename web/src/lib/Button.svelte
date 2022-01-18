<!--  Good usage on translate: transformY  -->
<script lang="ts" context="module">
</script>

<script lang="ts">
	export let href: string | null = null;
	export let type: string = 'button';
	export let disabled: boolean = false;

	const handleClick = (e: Event) => {
		if (disabled) {
			e.preventDefault();
		}
	};
</script>

<!-- REF https://www.joshwcomeau.com/animation/3d-button/  -->

{#if href}
	<a {href} class="govuk__button" on:click on:click={handleClick}>
		<span class="govuk__button__text" class:govuk__button__text--disabled={disabled}>
			<slot />
		</span>
	</a>
{:else}
	<button {type} class="govuk__button" on:click on:click={handleClick}>
		<span class="govuk__button__text" class:govuk__button__text--disabled={disabled}>
			<slot />
		</span>
	</button>
{/if}

<style lang="scss">
	.govuk__button {
		display: block;
		width: fit-content;
		color: #ffffff;
		border-radius: 5px;
		font-weight: 700;
		text-decoration: none;
		user-select: none;
		background-color: #000000;
        cursor: pointer;
	}

	.govuk__button__text {
		display: block;
		padding: 12px 26px;
		border-radius: 5px;
		background-color: #be132d;
		transform: translateY(-4px);
		min-width: 100px;
		text-align: center;
	}

	.govuk__button__text--disabled {
		background-color: #777777;
		cursor: not-allowed;
	}
</style>
