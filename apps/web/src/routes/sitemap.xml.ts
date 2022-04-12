import type { RequestHandler } from '@sveltejs/kit';
import env from '$lib/env';
import { ARTICLES_ROUTE } from '$const';

type BuildUrlArgs<T> = {
	pages: Array<T>;
	host: string;
};

type BuildUrl<T> = (args: BuildUrlArgs<T>) => Array<string>;

// https://blog.spotibo.com/sitemap-guide/
const buildUrl: BuildUrl<Article> = ({ pages, host }) =>
	pages.map((page) => {
		return `<url>
        <loc>https://${host}${page.slug}</loc>
		${
			page.image
				? `<image:image>
			<image:loc>https://${host}${page.image.src}</image:loc>
			<image:title>${page.image.alt}</image:title>
			<image:caption>${page.image.caption}</image:caption>
	 	</image:image>`
				: ''
		}
        <lastmod>${env.VITE_BUILD_TIME}</lastmod>
    </url>`;
	});

export const get: RequestHandler = async (req) => {
	try {
		const articlesRes = await fetch(req.url.origin + ARTICLES_ROUTE);
		if (!articlesRes.ok) {
			return {
				status: 500
			};
		}

		const articles = await articlesRes.json();

		const pages = [
			...articles,
			{
				slug: '/'
			}
		];

		return {
			headers: {
				'Content-Type': 'application/xml'
			},
			body: `
        <?xml version="1.0" encoding="UTF-8" ?>
        <urlset
          xmlns="https://www.sitemaps.org/schemas/sitemap/0.9"
          xmlns:xhtml="https://www.w3.org/1999/xhtml"
          xmlns:mobile="https://www.google.com/schemas/sitemap-mobile/1.0"
          xmlns:news="https://www.google.com/schemas/sitemap-news/0.9"
          xmlns:image="https://www.google.com/schemas/sitemap-image/1.1"
          xmlns:video="https://www.google.com/schemas/sitemap-video/1.1"
        >
        ${buildUrl({
					pages,
					host: req.url.host
				}).join('')}
        </urlset>
      `.trim()
		};
	} catch (error) {
		console.error(error);
		return {
			status: 500
		};
	}
};