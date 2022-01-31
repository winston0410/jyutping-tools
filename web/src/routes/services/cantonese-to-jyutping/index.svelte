<script lang="ts" context="module">
	import MetaData from '$lib/MetaData.svelte';
	import Input from '$lib/inputs/Input.svelte';
	import OutputArea from '$lib/Output.svelte';
	import Button from '$lib/Button.svelte';
	import { createForm } from 'felte';
	import type { ValidatorConfig } from '@felte/validator-zod';
	import { PROXY_ROOT, MICROSERVICE_ROOT, CONVERT_ACTION } from '$lib/const';
	import * as z from 'zod';
	import { validator } from '@felte/validator-zod';
	import { onMount } from 'svelte';
	import { hasCantonese, isCantoneseOnly, isTraditionalOnly, hasNumber } from '$lib/helper';
	import mkToast from '$lib/toast';
	import { getNotificationsContext } from 'svelte-notifications';
	import { TargetPhoneticSystem } from '$lib/types';
	import { extractPhonetic } from '$lib/format';
	//  import SideBarLayout from '$lib/layouts/SideBarLayout.svelte';

	export const load = async ({ fetch, url }) => {
		const { searchParams } = url;
		const input = searchParams.get('input');

		return {
			props: {}
		};
	};
</script>

<script lang="ts">
	export let result = '';

	//  Not using the enum now
	const { addNotification } = getNotificationsContext();
	const toast = mkToast(addNotification);

	const schema = z.object({
		'convert-characters': z.string().nonempty().refine(hasCantonese, {
			message: 'No cantonese found in the input'
		})
		// REF https://github.com/colinhacks/zod/issues/8
		//  to: z.nativeEnum(TargetPhoneticSystem)
	});

	const warnSchema = z.object({
		'convert-characters': z
			.string()
			.refine(hasNumber, {
				message: 'Arabic number might make the converter fail.'
			})
			.refine(isCantoneseOnly, {
				message: 'Non chinese characters might make the converter fail.'
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
			const payload = {
				input: values['convert-characters']
			};

			const res = await fetch(
				MICROSERVICE_ROOT + CONVERT_ACTION + `?${new URLSearchParams(payload)}`
			);

			if (!res.ok) {
				console.log('handle err', res);
				toast.mkError('Something wrong!');
			}

			// TODO Add typing for json response, typing avaliable in types.ts
			const body = await res.json();

			const phonetics = extractPhonetic(body.results);

			console.log('check phonetics', phonetics);

			result = phonetics.join(' ');

			toast.mkOk('Conversion successful!');
		},
		onError: (_: Error) => {
			// TODO Differentiate errors with err.message and err.name
			if (navigator.onLine) {
				toast.mkError('Service is temporaily unavailable.');
			} else {
				toast.mkError('You are not connected with the Internet.');
			}
		},
		extend: validator,
		validateSchema: schema,
		warnSchema
	});

	onMount(async () => {
		if ($data['convert-characters']) {
			await validate();
		}
	});

	const id = 'cantonese-to-jyutping';
</script>

<MetaData title="Jyutping converter" description="This is the description" url="" image="" />
<h1>Cantonese to Jyutping</h1>
<form use:form class="sidebar-layout" {id}>
	<!-- TODO Investigate how to leak style all component  -->
	<Input
		type="textarea"
		name={'convert-characters'}
		errors={$errors}
		warnings={$warnings}
		touched={$touched}
		spellcheck={false}
		placeholder="我係香港人"
		on:input={async () => {
			await validate();
		}}
	>
		<span>Cantonese</span>
	</Input>

	<div class="submit-button">
		<Button type="submit" disabled={!$isValid}>Convert</Button>
	</div>
</form>

<OutputArea
	{result}
	systems={Object.values(TargetPhoneticSystem)}
	form={id}
	placeholder={'ngo5 hai6 hoeng1 gong2 jan4'}
	on:copy={() => {
		toast.mkOk('Copied result to clipboard.');
	}}
/>

<style lang="scss">
	.submit-button {
		@include center;
		margin: 1.5rem 0;
	}
</style>
