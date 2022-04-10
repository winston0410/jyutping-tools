import type { RequestHandler } from '@sveltejs/kit';
import env from '$lib/env';

const articles: Array<Article> = [

];

const defaultQuery = {
	page: 1,
	offset: 10
};

export const get: RequestHandler = async ({ url }) => {
	const { page, offset } = {
		...Object.fromEntries(url.searchParams),
		...defaultQuery
	};

	return {
		body: articles.slice(offset * (page - 1), offset * page).reverse()
	};
};