<script lang="ts" context="module">
	export async function load() {
		return {
			status: 302,
			redirect: '/services/cantonese-to-jyutping'
		};
	}
</script>

<script lang="ts">
	const links = [
		{
			name: 'Segment Cantonese',
			href: '/services/segment-cantonese'
		},
		{
			name: 'Cantonese to Jyutping',
			href: '/services/cantonese-to-jyutping'
		}
	];
</script>

<!--  <h1>Jyutping Tools</h1>  -->
<!--  <p>This website contains tools to help you process Cantonese for learning purposes.</p>  -->
<!--  <ul role="list">  -->
<!--  {#each links as { name, href }}  -->
<!--  <li>  -->
<!--  <a {href} sveltekit:prefetch>  -->
<!--  {name}  -->
<!--  </a>  -->
<!--  </li>  -->
<!--  {/each}  -->

<!--  </ul>  -->
<style lang="scss">
</style>
