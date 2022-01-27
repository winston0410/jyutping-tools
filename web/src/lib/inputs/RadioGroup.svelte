<script lang="ts" context="module">
	import storage from 'svelte-use-local-storage';
</script>

<script lang="ts">
	export let values: Array<string>;
	export let name: string;
	export let checked = '';
</script>

<!-- REF https://accessibility.blog.gov.uk/2016/07/22/using-the-fieldset-and-legend-elements/  -->
<!-- REF https://www.w3.org/WAI/tutorials/forms/grouping/  -->
<fieldset use:storage={name}>
	<legend>
		<slot />
	</legend>

	{#each values as value}
		<div>
			<!-- REF https://github.com/sveltejs/rfcs/pull/33  -->
			<!--  {@const id = `${id}-${value}`}          -->
			<label for={`${name}-${value}`}>
				<span class="radio-name">
					{value}
				</span>
				<input
					type="radio"
					id={`${name}-${value}`}
					{name}
					{value}
					checked={value === checked}
					on:input
					on:change
				/>
			</label>
		</div>
	{/each}
</fieldset>

<style lang="scss">
	.radio-name {
		text-transform: capitalize;
	}
</style>
