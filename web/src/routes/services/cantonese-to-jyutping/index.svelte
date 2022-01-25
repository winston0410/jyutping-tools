<script lang="ts" context="module">
	import MetaData from '$lib/MetaData.svelte';
	import Input from '$lib/Input.svelte';
	import Button from '$lib/Button.svelte';
	import Separator from '$lib/Separator.svelte';
	import { createForm } from 'felte';
	import type { ValidatorConfig } from '@felte/validator-zod';
	import { PROXY_ROOT, MICROSERVICE_ROOT, CONVERT_ACTION } from '$lib/const';
	import * as z from 'zod';
	import { validator } from '@felte/validator-zod';
	import { onMount } from 'svelte';
	import { isCantoneseOnly, isTraditionalOnly } from '$lib/helper';
	import mkToast from '$lib/toast';
	import { getNotificationsContext } from 'svelte-notifications';

	export const load = async ({ fetch }) => {
		return {
			props: {}
		};
	};
</script>

<script lang="ts">
	const { addNotification } = getNotificationsContext();
	const toast = mkToast(addNotification);

	const schema = z.object({
		'convert-characters': z.string().nonempty()
	});

	const warningSchema = z.object({
		'convert-characters': z.string().refine(isCantoneseOnly, {
			message: 'Non chinese characters might make the translator fail.'
		})
		//  .refine(isTraditggionalOnly, {
		//  message: 'Simplified characters might make the translator fail.'
		//  })
	});

	const { form, errors, touched, validate, data, isValid } = createForm<
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
			}

			const body = await res.json();

			console.log('result', body);
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
		warningSchema
	});

	onMount(async () => {
		if ($data['convert-characters']) {
			await validate();
		}
	});
</script>

<MetaData title="Jyutping converter" description="This is the description" url="" image="" />

<h1>Cantonese to Jyutping</h1>
<form use:form>
	<!--  warnings={$warnings}  -->
	<div class="layout">
		<Input
			type="textarea"
			name={'convert-characters'}
			error={$errors}
			touched={$touched}
			spellcheck={false}
			placeholder="我係香港人"
			on:input={async () => {
				await validate();
			}}
		>
			<span>Cantonese</span>
		</Input>
		<Separator />

		<div class="control-panel">
			<Button type="submit" disabled={!$isValid}>Convert</Button>
		</div>
	</div>
</form>

<style lang="scss">
    .layout {
    }

	.control-panel {
	}
</style>
