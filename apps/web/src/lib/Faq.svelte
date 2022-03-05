<script lang="ts" context="module">
</script>

<script lang="ts">
	export let entity: FAQEntity;
	// NOTE Dynamic element is not supported in Svelte yet
	export let questionTag: string;
</script>

<section class="faq" itemscope itemtype="https://schema.org/Question" itemprop="mainEntity">
	{#if questionTag === 'h2'}
		<h2 itemprop="name">
			{entity.question}
		</h2>
	{:else if questionTag === 'h3'}
		<h3 itemprop="name">
			{entity.question}
		</h3>
	{/if}
	<div itemscope itemprop="acceptedAnswer" itemtype="https://schema.org/Answer">
		<p itemprop="text">
			{entity.answer}
		</p>
	</div>
</section>

<style lang="scss">
	.faq {
		height: 100%;
		@include card;
		padding: 1rem;
	}
</style>
