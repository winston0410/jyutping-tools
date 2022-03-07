<script lang="ts" context="module">
	import { DevelopmentPhrase } from '$lib/types';
</script>

<script lang="ts">
	export let phase = DevelopmentPhrase.Alpha;
</script>

<div class="phase-banner">
		{#if phase === DevelopmentPhrase.Alpha}
			<span class="icon">Alpha</span>
		{:else if phase === DevelopmentPhrase.Beta}
			<span class="icon">Beta</span>
		{/if}
		<span class="text">This service is under active development and testing. Please
		<a class="link" rel="external" href="https://github.com/winston0410/jyut.info/issues/new"
			>give us your feedback</a
		>.</span>
</div>
<hr class="break"/>

<style lang="scss">
	.text {
        margin-left: 0.5rem;
		@include tablet {
			margin-left: 1rem;
		}
	}

    .break{
        border: 0;
        height: 1px;
        background: #eee;
        margin-top: 1rem;
        margin-bottom: 1rem;
    }
</style>
