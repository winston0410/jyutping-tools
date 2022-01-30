import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async () => {
	const data = '';

	return {
		body: data
	};
};
