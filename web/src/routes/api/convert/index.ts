import type { RequestHandler } from '@sveltejs/kit';
import { MICROSERVICE_ROOT, CONVERT_ACTION, MICROSERVICE_CREDENTIAL } from '$lib/const';

type SelectedHeaders = {
	'cache-control': string;
	date: string;
	expires: string;
	'last-modified': string;
	vary: string;
};

const getCacheHeaders = (headers: Headers): SelectedHeaders => {
	return {
		'cache-control': headers.get('cache-control'),
		date: headers.get('date'),
		expires: headers.get('expires'),
		'last-modified': headers.get('last-modified'),
		vary: headers.get('vary')
	};
};

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
			headers: {
				...getCacheHeaders(res.headers)
			},
			body: await res.json()
		};
	} catch (e) {
		return {
			status: 500
		};
	}
};
