//  Check others implementations
// REF https://github.com/sveltejs/kit/issues/1142
import type { RequestHandler } from '@sveltejs/kit';
import env from '$lib/env';

const PAGE_LIST = ['/services/cantonese-to-jyutping'];

type BuildUrlArgs = {
	urls: Array<string>;
	host: string;
};

type BuildUrl = (args: BuildUrlArgs) => Array<string>;

const buildUrl: BuildUrl = ({ urls, host }: BuildUrlArgs) =>
	urls.map((url) => {
		return `<url>
        <loc>https://${host}${url}</loc>
        <lastmod>${env.VITE_BUILD_TIME}</lastmod>
    </url>`;
	});

export const get: RequestHandler = async (req) => {
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
				urls: PAGE_LIST,
				host: req.url.host
			})}
      </urlset>
    `.trim()
	};
};
