import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async (req) => {
	return {
		headers: {
			'Content-Type': 'text/plain'
		},
		body: `User-agent: *
Allow: /
Sitemap: https://${req.url.host}/sitemap.xml`
	};
};
