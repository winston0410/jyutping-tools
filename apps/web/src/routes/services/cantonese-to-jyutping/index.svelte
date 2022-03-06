<script lang="ts" context="module">
	import MetaData from '$lib/MetaData.svelte';
	import Input from '$lib/inputs/Input.svelte';
	import OutputArea from '$lib/Output.svelte';
	import Button from '$lib/Button.svelte';
	import { createForm } from 'felte';
	import type { ValidatorConfig } from '@felte/validator-zod';
	import { PROXY_ROOT, CONVERT_ACTION, MICROSERVICE_ROOT } from '$lib/const';
	import * as z from 'zod';
	import { validator } from '@felte/validator-zod';
	import { hasCantonese, isCantoneseOnly, isTraditionalOnly, hasNumber } from '$lib/predicate';
	import mkToast from '$lib/toast';
	import { getNotificationsContext } from 'svelte-notifications';
	import { TargetPhoneticSystem, InvalidCode } from '$lib/types';
	import { extractPhonetic } from '$lib/format';
	import { handleReplaceArabicNumber } from '$lib/handler';
	import { page } from '$app/stores';
	import Faq from '$lib/Faq.svelte';
	import { onMount } from 'svelte';
    import PhaseBanner from '$lib/PhaseBanner.svelte'

	export const load = async ({ url, fetch }) => {
		const { searchParams } = url;
		const input = searchParams.get('input') ?? '';
		let result = null;
		let errorMessage = '';

		if (input) {
			const res = await fetch(
				MICROSERVICE_ROOT + CONVERT_ACTION + `?${searchParams.toString()}`
			).catch((e: unknown) => e);

			if (res.ok) {
				const body = await res.json();
				result = extractPhonetic(body.results);
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
			}
		};
	};
</script>

<script lang="ts">
	export let input: string;
	export let faqEntities: Array<FAQEntity>;
	export let result: Array<string> | null;
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

	const id = 'cantonese-to-jyutping';
	const textareaName = 'convert-characters';
	const outputName = 'romanization';
	let textareaRef: HTMLTextAreaElement;

	$: errorCode = $errors[textareaName]?.[0];
	$: warningCode = $warnings[textareaName]?.[0];
</script>

<MetaData
	title="Cantonese to romanization | Cantonese.tools"
	description="Convert Cantonese characters to romanizations including Jyutping and Yale here with our performant Rust Cantonese NLP engine."
	url=""
	image=""
/>
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
			<div class="error-message">
				{#if errorCode === InvalidCode.NoCantoneseCharacter}
					<span class="error">No Cantonese characters.</span>
				{:else if errorCode === InvalidCode.EmptySelfInput}
					<span class="error">The input field is empty.</span>
				{/if}
			</div>
		</div>
		<div slot="warning">
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
	{result}
	systems={Object.values(TargetPhoneticSystem)}
	form={id}
	placeholder={'ngo5 hai6 hoeng1 gong2 jan4'}
	on:copy={() => {
		toast.mkOk('Copied result to clipboard');
	}}
/>

<div class="lg-separator" />

<section>
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
</style>
