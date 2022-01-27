<script lang="ts" context="module">
	import { TargetPhoneticSystem } from '$lib/types';
	import mkTabs from '$lib/actions/mkTabs';
	import clipboard from '$lib/actions/clipboard';
	//  @ts-ignore
	import CopyIcon from 'virtual:icons/carbon/copy';
</script>

<script lang="ts">
	// REF https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output#attr-form
	export let form: string;
	export let result: string;
	export let systems: Array<TargetPhoneticSystem>;

	const { tab, currentTab } = mkTabs({
		initial: TargetPhoneticSystem.Jyutping
	});

	let outputElem: HTMLOutputElement;
	//  let outputElem = "hello";
</script>

<ul role="list" class="switcher">
	{#each systems as system}
		<li><button class="switcher-button" type="button" use:tab={system}>{system}</button></li>
	{/each}
</ul>
<button type="button" use:clipboard={outputElem}>
	<CopyIcon />
</button>
<output {form} class="" bind:this={outputElem}>
	{#if $currentTab === TargetPhoneticSystem.Jyutping}
		{$currentTab}
		{result}
	{:else if $currentTab === TargetPhoneticSystem.Yale}
		{$currentTab}
		{result}
	{/if}
</output>

<style lang="scss">
	.switcher {
		display: flex;
		flex-direction: row;
		justify-content: flex-end;
		padding: 0;
	}

	.switcher-button {
		padding: 12px;
		text-transform: capitalize;
	}
</style>
