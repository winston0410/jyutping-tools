<script lang="ts" context="module">
</script>

<script lang="ts">
	export let name: string;
	export let type: string;
	export let value: string;
	export let checked = false;

	const id = `${name}-${value}`;
</script>

<div>
	<label for={id}>
		<span class="radio-name">
			{value}
		</span>
		<input type="radio" {id} {name} {value} {checked} on:input on:change />
	</label>
</div>

<style lang="scss">
	.radio-name {
		text-transform: capitalize;
	}
</style>
