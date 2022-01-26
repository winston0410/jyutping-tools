<script lang="ts" context="module">
	import Radio from './Radio.svelte';
	//  import storage from './debug';
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
		<Radio {name} type="radio" {value} checked={checked === value} />
	{/each}
</fieldset>
