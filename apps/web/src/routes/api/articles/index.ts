import type { RequestHandler } from '@sveltejs/kit';
import env from '$lib/env';
import WiseLogoSrcset from '$assets/wise-logo.jpg?w=300;768;1200&format=webp&srcset';
import WiseLogo from '$assets/wise-logo.jpg?metadata';

const articles: Array<Article> = [
	{
		title: 'How to say "I love you" in Cantonese?',
		description: 'Do you know how to say I love you in Cantonese?',
		slug: '/articles/how-to-say-i-love-you-in-Cantonese',
		fileName: 'how-to-say-i-love-you',
		image: {
			src: WiseLogo.src,
			srcset: WiseLogoSrcset,
			format: WiseLogo.format,
			alt: 'Wise 嘅 Logo',
			caption: '今次就教吓大家點用Wise零手續費、中間價匯率電匯去英國。',
			width: WiseLogo.width,
			height: WiseLogo.height
		},
		created_at: '2020-04-03T08:39:07:z',
		updated_at: env.VITE_BUILD_TIME as string,
		keywords: ['I love you in Cantonese']
	},
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