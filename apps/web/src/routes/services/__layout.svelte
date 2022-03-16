<script lang="ts" context="module">
    // FIXME somehow ESM import doesn't work properly, has to use default import
    // FIXME The adblock detection cannot detect Brave shield
	//  import * as detect from 'just-detect-adblock';
	import { onMount } from 'svelte';
</script>

<script lang="ts">
	onMount(async () => {
        try {
            //  const hasAdBlock = await detect.default.detectAnyAdblocker();
            //  console.log('check blocker', hasAdBlock)
        } catch (e) {
            console.log('check error', e)
        }
	});
</script>

<slot />
