import adapter from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import Icons from 'unplugin-icons/vite';

const filePath = dirname(fileURLToPath(import.meta.url));
const sassPath = `${filePath}/src/lib`;
/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		immutable: true
	},
	preprocess: preprocess({
		scss: {
			prependData: `@import '${sassPath}/scss/vars.scss';`
		}
	}),

	kit: {
		adapter: adapter(),
		target: '#svelte',
		vite: {
			build: {
				minify: true
			},
			plugins: [
				Icons({
					compiler: 'svelte'
				})
			]
		}
	}
};

export default config;
