<script lang="ts" context="module">
	import { TargetPhoneticSystem } from '$lib/types';
	import mkTabs from '$lib/actions/mkTabs';
	import clipboard from '$lib/actions/clipboard';
	//  @ts-ignore
	import CopyIcon from 'virtual:icons/carbon/copy';
	import { jyutpingToYale, jyutpingToTraditionalYale } from 'jyutping-helpers';
</script>

<script lang="ts">
	// REF https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output#attr-form
	export let form: string;
	export let name: string;
	export let result: Array<string> | null;
	export let systems: Array<TargetPhoneticSystem>;
	export let placeholder: string = '';

	const { tab, currentTab } = mkTabs({
		initial: TargetPhoneticSystem.Jyutping
	});
</script>

<div class="output-header">
	<label for={name} class="label">Romanization</label>

	<ul role="list" class="switcher">
		{#each systems as system}
			<li>
				<button
					class="switcher-button"
					class:active={$currentTab === system}
					type="button"
					use:tab={system}>{system}</button
				>
			</li>
		{/each}
	</ul>
</div>
<div class="output-wrapper">
	<button class="overlay-button" type="button">
		<CopyIcon class="copy-icon" />
	</button>
	<div class="output">
		<output {name} {form} class="output-inner" class:placeholder={!result} use:clipboard on:copy>
			{#if $currentTab === TargetPhoneticSystem.Jyutping}
				{result ? result.join(' ') : placeholder}
			{:else if $currentTab === TargetPhoneticSystem.ToneNumberYale}
				{result ? jyutpingToYale(result).join(' ') : placeholder}
			{:else if $currentTab === TargetPhoneticSystem.ToneMarkYale}
				{result ? jyutpingToTraditionalYale(result).join(' ') : placeholder}
			{/if}
		</output>
	</div>
</div>
<!--  :global(.copy-icon path) {  -->
<!--  transition: fill 300ms;  -->
<!--  }  -->
<!--  :global(.copy-icon:hover path) {  -->
<!--  fill: #be132d;  -->

<!--  }  -->
<style lang="scss">
	.output-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.switcher {
		display: flex;
		flex-direction: row;
		justify-content: flex-end;
		padding: 0;
		margin: 0;
	}

	.switcher-button {
		@include center;
		padding: 12px;
		text-transform: capitalize;
		min-width: 100px;
		transition: background 300ms;
        height: $input-header-height;
	}

	.switcher-button:hover {
		background: darken(#be132d, 20%);
	}

	.active {
		background: #be132d;
	}

	.active:hover {
		background: darken(#be132d, 20%);
	}

	.overlay-button {
		@include center;
		position: absolute;
		right: 12px;
		top: 12px;
		background-color: $white;
	}

	.output-wrapper {
		position: relative;
		cursor: pointer;
	}

	.output {
		@include textarea;
		padding: 0;
		color: $black;
	}

	.placeholder {
		@include placeholder-text;
	}

	.output-inner {
		display: block;
		height: 100%;
		padding: 12px;
	}

	.label {
		font-weight: 700;
	}
</style>
