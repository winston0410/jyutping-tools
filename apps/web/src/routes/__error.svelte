<script context="module" lang="ts">
	import type { ErrorLoad } from '@sveltejs/kit';
    // TODO Set Meta
	export const load: ErrorLoad = ({ error, status }) => {
        if(status === 500){
            console.error(error)
        }
        
		return {
			props: {}
		};
	};
</script>

<script lang="ts">
</script>

<h1 class="title">Page Not Found!</h1>

<style lang="scss">
    .title {
        @include center;
    }
</style>
