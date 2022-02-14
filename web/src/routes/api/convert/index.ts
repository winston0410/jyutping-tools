import type { RequestHandler } from '@sveltejs/kit';
import { MICROSERVICE_ROOT, CONVERT_ACTION, MICROSERVICE_CREDENTIAL } from '$lib/const';

export const get: RequestHandler = async ({ url }) => {
	try {
		const res = await fetch(
			MICROSERVICE_ROOT + CONVERT_ACTION + `?${url.searchParams.toString()}`,
			{
				headers: { Authorization: MICROSERVICE_CREDENTIAL }
			}
		);

		if (!res.ok) {
			return {
				status: res.status
			};
		}

		return {
			body: await res.json()
		};
	} catch (e) {
		return {
			status: 500
		};
	}
};
