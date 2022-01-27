<script lang="ts" context="module">
	import { TargetPhoneticSystem } from '$lib/types';
	import mkTabs from '$lib/actions/mkTabs';
</script>

<script lang="ts">
	export let result: string;
	export let systems: Array<TargetPhoneticSystem>;

	const { tab, currentTab } = mkTabs({
		initial: TargetPhoneticSystem.Jyutping
	});
</script>

<ul role="list">
	{#each systems as system}
		<li><button use:tab={system}>{system}</button></li>
	{/each}
</ul>
<div>
	{#if $currentTab === TargetPhoneticSystem.Jyutping}
		{$currentTab}
		{result}
	{:else}
		{$currentTab}
		{result}
	{/if}
</div>

<style lang="scss">
</style>
