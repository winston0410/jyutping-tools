<script lang="ts" context="module">
</script>

<script lang="ts">
	// NOTE Using px as unit
	export let size: number = 16;
	export let horizontal = true;
	let direction = horizontal ? 'height' : 'width';
</script>

<div style={`${direction}: ${size / 16}rem`} />
