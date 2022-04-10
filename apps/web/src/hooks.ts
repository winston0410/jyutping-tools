import { minify } from 'html-minifier-terser';

export async function handle({ event, resolve }) {
	const response = await resolve(event);

	if (response.headers.get('content-type') === 'text/html') {
		return new Response(
			await minify(await response.text(), {
				collapseBooleanAttributes: true,
				collapseWhitespace: true,
				conservativeCollapse: true,
				decodeEntities: true,
				html5: true,
				minifyCSS: true,
				minifyJS: false,
				removeAttributeQuotes: true,
				removeComments: true,
				removeOptionalTags: true,
				removeRedundantAttributes: true,
				removeScriptTypeAttributes: true,
				removeStyleLinkTypeAttributes: true,
				sortAttributes: true,
				sortClassName: true
			}),
			{
				status: response.status,
				headers: response.headers
			}
		);
	}

	return response;
}