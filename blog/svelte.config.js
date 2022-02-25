import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import { mdsvex } from 'mdsvex';
import { imagetools } from 'vite-imagetools';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [mdsvex({ extensions: ['.md'] }), preprocess()],

	kit: {
		adapter: adapter(),
		vite: {
			plugins: [imagetools({ force: true })]
		}
	}
};

export default config;
