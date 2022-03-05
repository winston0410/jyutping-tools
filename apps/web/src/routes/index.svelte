<script lang="ts" context="module">
	import type { Load } from '@sveltejs/kit';

	export const load: Load = () => {
		return {
			redirect: '/services/cantonese-to-jyutping',
			status: 302
		};
	};
</script>

<script lang="ts">
</script>

<style lang="scss">
</style>
