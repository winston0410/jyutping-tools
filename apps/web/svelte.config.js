import adapter from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import Icons from 'unplugin-icons/vite';
import { mdsvex } from 'mdsvex';
import { imagetools } from 'vite-imagetools';

const filePath = dirname(fileURLToPath(import.meta.url));
const sassPath = `${filePath}/src/lib`;
const mdsvexExtension = ['.svelte', '.svelte.md'];
/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		immutable: true
	},
	extensions: mdsvexExtension,
	preprocess: [
		mdsvex({ extensions: mdsvexExtension }),
		preprocess({
			scss: {
				prependData: `@import '${sassPath}/scss/vars.scss';`
			}
		})
	],
	kit: {
		adapter: adapter(),
		vite: {
			build: {
				minify: true
			},
			plugins: [
				Icons({
					compiler: 'svelte'
				}),
				imagetools({ force: true })
			]
		}
	}
};

export default config;
