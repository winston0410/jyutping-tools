<script lang="ts" context="module">
	export const load = async ({ fetch }) => {
		return {
			status: 302,
			redirect: '/jyutping-to-cantonese'
		};
	};
</script>
