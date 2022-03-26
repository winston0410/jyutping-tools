<script lang="ts" context="module">
	import type { ResultTuple } from '$lib/types';
	import mkTabs from '$lib/actions/mkTabs';
	import Input from '$lib/inputs/Input.svelte';
	import OutputArea from '$lib/Output.svelte';
	import Button from '$lib/Button.svelte';
	import { createForm } from 'felte';
	import type { ValidatorConfig } from '@felte/validator-zod';
	import { PROXY_ROOT, PARSE_ACTION, MICROSERVICE_ROOT } from '$lib/const';
	import * as z from 'zod';
	import { validator } from '@felte/validator-zod';
	import { hasCantonese, isCantoneseOnly, hasNumber } from '$lib/predicate';
	import mkToast from '$lib/toast';
	import { getNotificationsContext } from 'svelte-notifications';
	import { TargetPhoneticSystem, InvalidCode } from '$lib/types';
	import { handleReplaceArabicNumber } from '$lib/handler';
	import { page } from '$app/stores';
	import Faq from '$lib/Faq.svelte';
	import { onMount } from 'svelte';
	import { jyutpingToYale, jyutpingToTraditionalYale } from 'jyutping-helpers';
	import PhaseBanner from '$lib/PhaseBanner.svelte';

	export const load = async ({ url, fetch }) => {
		const { searchParams } = url;
		const input = searchParams.get('input') ?? '';
		let result = null;
		let errorMessage = '';

		if (input) {
			const res = await fetch(
				MICROSERVICE_ROOT + PARSE_ACTION + `?${searchParams.toString()}`
			).catch((e: unknown) => e);

			if (res.ok) {
				const body = await res.json();
				result = body.results;
			} else {
				errorMessage = 'Service is temporaily unavailable';
			}
		}

		const faqEntities = await (await fetch(PROXY_ROOT + '/ui/faq')).json();

		return {
			props: {
				input,
				result,
				errorMessage,
				faqEntities
			},
			stuff: {
				title: 'Cantonese to romanization | jyut.info',
				description:
					'Convert Cantonese characters to romanizations including Jyutping and Yale here with our performant Rust Cantonese NLP engine.',
				image: ''
			}
		};
	};
</script>

<script lang="ts">
	export let input: string;
	export let faqEntities: Array<FAQEntity>;
	export let result: Array<ResultTuple> | null;
	export let errorMessage: string;

	let outputElem: HTMLOutputElement;

	const { addNotification } = getNotificationsContext();
	const toast = mkToast(addNotification);

	onMount(() => {
		if (errorMessage) {
			toast.mkError(errorMessage);
		}

		if (result?.length > 0) {
			outputElem.scrollIntoView({
				behavior: 'smooth'
			});
			toast.mkOk('Conversion successful');
		}
	});

	const schema = z.object({
		'convert-characters': z
			.string()
			.nonempty({
				message: InvalidCode.EmptySelfInput
			})
			.refine(hasCantonese, {
				message: InvalidCode.NoCantoneseCharacter
			})
		// REF https://github.com/colinhacks/zod/issues/8
		//  to: z.nativeEnum(TargetPhoneticSystem)
	});

	const warnSchema = z.object({
		'convert-characters': z
			.string()
			.refine(hasNumber, {
				message: InvalidCode.FoundArabicNumber
			})
			.refine(isCantoneseOnly, {
				message: InvalidCode.FoundNonCantoneseCharacter
			})
		//  .refine(isTraditggionalOnly, {
		//  message: 'Simplified characters might make the translator fail.'
		//  })
	});

	const { form, errors, touched, validate, data, isValid, warnings } = createForm<
		z.infer<typeof schema>,
		ValidatorConfig
	>({
		onSubmit: async (values) => {
			const input = values['convert-characters'];

			const payload = {
				input
			};

			const query = `?${new URLSearchParams(payload)}`;

			const currentPageUrl = $page.url.origin + $page.url.pathname;

			// NOTE For server-side rendering
			window.location.href = currentPageUrl + query;
		},
		extend: validator,
		validateSchema: schema,
		warnSchema
	});

	const { tab, currentTab } = mkTabs({
		initial: TargetPhoneticSystem.Jyutping
	});

	const id = 'cantonese-to-jyutping';
	const textareaName = 'convert-characters';
	const outputName = 'romanization';
	let textareaRef: HTMLTextAreaElement;

	let hasUnknown = result?.findIndex(([, jyutping]) => !jyutping) >= 0;

	$: errorCode = $errors[textareaName]?.[0];
	$: warningCode = $warnings[textareaName]?.[0];
</script>

<PhaseBanner />
<div class="lg-separator" />
<h1>Cantonese to romanization</h1>
<p>Convert Cantonese characters to romanization easily, supporting Jyutping and Yale.</p>
<form use:form class="sidebar-layout" {id}>
	<Input
		type="textarea"
		name={textareaName}
		bind:ref={textareaRef}
		errors={$errors}
		warnings={$warnings}
		touched={$touched}
		spellcheck={false}
		placeholder="我係香港人"
		initValue={input}
		on:input={async () => {
			await validate();
		}}
	>
		<span>Cantonese</span>
		<div slot="error">
			<div class="sm-separator" />
			<div class="error-message">
				{#if errorCode === InvalidCode.NoCantoneseCharacter}
					<span class="error">No Cantonese characters.</span>
				{:else if errorCode === InvalidCode.EmptySelfInput}
					<span class="error">The input field is empty.</span>
				{/if}
			</div>
		</div>
		<div slot="warning">
			<div class="sm-separator" />
			<div class="error-message">
				{#if warningCode === InvalidCode.FoundArabicNumber}
					<span class="warning"
						>Arabic numbers might yield unexpected result.
						<button class="link" type="button" on:click={handleReplaceArabicNumber(textareaRef)}
							>Convert all arabic numbers to Cantonese numbers</button
						>.</span
					>
				{:else if warningCode === InvalidCode.FoundNonCantoneseCharacter}
					<span class="warning"
						>Non Cantonese characters might yield unexpected result.
						<!--  <button class="link" type="button">Strip off all non Cantonese characters</button>  -->
					</span>
				{/if}
			</div>
		</div>
	</Input>

	<div class="submit-button">
		<Button type="submit" disabled={!$isValid}>Convert</Button>
	</div>
</form>

<OutputArea
	id="romanization"
	bind:ref={outputElem}
	name={outputName}
	form={id}
	placeholder={'ngo5 hai6 hoeng1 gong2 jan4'}
	on:copy={() => {
		toast.mkOk('Copied result to clipboard');
	}}
>
	<div slot="header">
		<ul role="list" class="switcher">
			{#each Object.values(TargetPhoneticSystem) as system}
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
	<div slot="output">
		{#if result}
			{#each result as [key, tokens]}
				{#if !tokens}
					{`unknown(${key})`}
				{:else}
					{#each tokens as token}
						{#if typeof token === 'string'}
							{token}
						{:else}
							<span class={`pos-${token.pos}`}>
								{#if $currentTab === TargetPhoneticSystem.Jyutping}
									{token.jyutping}
								{:else if $currentTab === TargetPhoneticSystem.ToneNumberYale}
									{jyutpingToYale(token.jyutping)}
								{:else if $currentTab === TargetPhoneticSystem.ToneMarkYale}
									{jyutpingToTraditionalYale(token.jyutping)}
								{/if}
							</span>
						{/if}
					{/each}
				{/if}
			{/each}
		{/if}
	</div>
	<div slot="warning">
		{#if hasUnknown}
			<div class="sm-separator" />
			<div class="error-message">
				<span class="warning"
					>Do you know what those unknown value might be? <a
						class="link"
						rel="external"
						href={`https://github.com/winston0410/jyut.info/issues/new?assignees=&labels=romanization&title=${encodeURIComponent(
							`Unknown found: ${input}`
						)}&body=${encodeURIComponent(`Current output: ${result.join(' ')}
Expected output: 
`)}`}>Help us improve this converter.</a
					></span
				>
			</div>
		{/if}
	</div>
</OutputArea>

<div class="lg-separator" />

<section itemscope itemtype="https://schema.org/FAQPage">
	<h2>Frequently Asked Questions</h2>
	<ul role="list" class="faq-list">
		{#each faqEntities as entity (entity.question)}
			<li>
				<Faq {entity} questionTag="h3" />
			</li>
		{/each}
	</ul>
</section>

<style lang="scss">
	.submit-button {
		@include center;
		margin: 1.5rem 0;
	}

	.error-message {
		@include input-header-layout;
	}

	.faq-list {
		display: grid;
		padding: 0;
		row-gap: 2rem;
		@include tablet {
			grid-template-columns: repeat(2, 1fr);
			column-gap: 1rem;
		}
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
		@include card-shadow;
		padding: 12px;
		text-transform: capitalize;
		transition: background 300ms, color 300ms;
		height: $input-header-height;
		border-top-left-radius: 0.25rem;
		border-top-right-radius: 0.25rem;
		background-color: var(--color-unselected-button);
		@include tablet {
			min-width: 8rem;
		}
	}

	.switcher-button:not(.active):hover {
		color: var(--color-contrast-text);
		background: var(--color-button-highlight);
	}

	.active {
		cursor: default;
		color: var(--color-contrast-text);
		background: var(--color-button);
	}
</style>
