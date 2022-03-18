<script lang="ts" context="module">
	import clipboard from '$lib/actions/clipboard';
	//  @ts-ignore
	import CopyIcon from 'virtual:icons/carbon/copy';
</script>

<script lang="ts">
	//  For scrolling into view
	export let ref = null;
	export let id: string;
	// REF https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output#attr-form
	export let form: string;
	export let name: string;
	export let placeholder: string = '';
</script>

<div class="output-header">
	<label for={name} class="label">Romanization</label>

	<slot name="header" />

	<!--  <ul role="list" class="switcher">  -->
	<!--  {#each systems as system}  -->
	<!--  <li>  -->
	<!--  <button  -->
	<!--  class="switcher-button"  -->
	<!--  class:active={$currentTab === system}  -->
	<!--  type="button"  -->
	<!--  use:tab={system}>{system}</button  -->
	<!--  >  -->
	<!--  </li>  -->
	<!--  {/each}  -->
	<!--  </ul>  -->
</div>
<div class="output-wrapper">
	<button class="overlay-button" type="button">
		<CopyIcon class="copy-icon" />
	</button>
	<div class="output">
		<!-- TODO get reference to a slot?  -->
		<!--  class:placeholder={!result}  -->
		<output bind:this={ref} {id} {name} {form} class="output-inner" use:clipboard on:copy>
			<slot name="output" />
		</output>
	</div>
</div>
<slot name="warning" />
<!--  :global(.copy-icon path) {  -->
<!--  transition: fill 300ms;  -->
<!--  }  -->
<!--  :global(.copy-icon:hover path) {  -->
<!--  fill: #be132d;  -->

<!--  }  -->
<style lang="scss">
	.output-header {
		display: flex;
		flex-direction: column;
		row-gap: 1rem;
		@include tablet {
			flex-direction: row;
			row-gap: 0rem;
			align-items: center;
			justify-content: space-between;
		}
	}

	.overlay-button {
		@include center;
		position: absolute;
		right: 1rem;
		top: 1rem;
		background-color: $white;
	}

	.output-wrapper {
		position: relative;
		cursor: pointer;
		border-radius: 0.25rem;
	}

	.output {
		@include textarea;
		padding: 0;
		color: $black;
	}

	.output-wrapper:hover {
		outline: $outline-width dashed var(--color-text);
	}

	.placeholder {
		@include placeholder-text;
	}

	.output-inner {
		display: block;
		height: 100%;
		padding: 1rem 3rem 1rem 1rem;
	}

	.label {
		font-weight: 700;
	}
</style>
