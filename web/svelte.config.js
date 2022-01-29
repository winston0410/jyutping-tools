import adapter from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import Icons from 'unplugin-icons/vite';

const filePath = dirname(fileURLToPath(import.meta.url));
const sassPath = `${filePath}/src/lib`;
/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess({
		scss: {
			prependData: `@import '${sassPath}/scss/vars.scss';`
		}
	}),

	kit: {
		adapter: adapter(),

		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		vite: {
			plugins: [
				Icons({
					compiler: 'svelte'
				})
			]
		}
	}
};

export default config;
