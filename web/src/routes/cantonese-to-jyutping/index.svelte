<script lang="ts" context="module">
	import MetaData from '$lib/MetaData.svelte';
	import Input from '$lib/Input.svelte';
	import Button from '$lib/Button.svelte';
	import { createForm } from 'felte';
	import { PROXY_ROOT, MICROSERVICE_ROOT, CONVERT_ACTION } from '$lib/const';
	import * as z from 'zod';
	import { validator } from '@felte/validator-zod';

	export const load = async ({ fetch }) => {
		return {
			props: {}
		};
	};
</script>

<script lang="ts">
	const schema = z.object({
		'convert-characters': z.string().nonempty()
	});

	const { form, errors, validate } = createForm({
		onSubmit: async (values) => {
			const payload = {
				input: values['convert-characters']
			};
			const res = await fetch(
				MICROSERVICE_ROOT + CONVERT_ACTION + `?${new URLSearchParams(payload)}`
			);

			if (!res.ok) {
				console.log('handle err');
			}

			const body = await res.json();

			console.log('result', body);
		},
		onError: (err) => {
			console.log(err);
		},
		extend: validator,
		validateSchema: schema
	});
</script>

<MetaData title="Jyutping converter" description="This is the description" url="" image="" />
<h1>Cantonese to Jyutping</h1>
<form use:form>
	<Input
        type="textarea"
		name={'convert-characters'}
        error={$errors['convert-characters']}
		on:input={async () => {
			await validate();
		}}
	>
		<span>Cantonese</span>
	</Input>
	<Button type="submit">Convert</Button>
</form>
