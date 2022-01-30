<script lang="ts" context="module">
</script>

<script lang="ts">
</script>

<!-- NOTE Cannot use __layout as a component right now  -->
<!-- REF https://github.com/sveltejs/kit/issues/627  -->
<!--  <slot name="form"/>  -->
<!--  <slot name="panel"/>  -->
<!--  <slot name="rest"/>  -->
<div class="shell">
	<slot />
</div>

<style lang="scss">
	.shell {
		padding: 20px;
	}
</style>
