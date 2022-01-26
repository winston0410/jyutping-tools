<script lang="ts" context="module">
	import Radio from './Radio.svelte';
</script>

<script lang="ts">
	export let values: Array<string>;
	export let name: string;
	export let checked = '';
</script>

<!-- REF https://accessibility.blog.gov.uk/2016/07/22/using-the-fieldset-and-legend-elements/  -->
<fieldset>
	<legend>
		<slot />
	</legend>

	{#each values as value}
		<Radio {name} type="radio" {value} checked={checked === value} />
	{/each}
</fieldset>
