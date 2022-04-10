import adapter from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';
import { dirname, parse, resolve } from 'path';
import { fileURLToPath } from 'url';
import Icons from 'unplugin-icons/vite';
import { mdsvex } from 'mdsvex';
import { imagetools } from 'vite-imagetools';

const filePath = dirname(fileURLToPath(import.meta.url));
const libPath = `${filePath}/src/lib`;
const extensions = ['.svelte', '.svelte.md'];

/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		immutable: true
	},
	extensions,
	preprocess: [
		mdsvex({
			extensions: extensions.slice(1),
			layout: {
				article: resolve('src', 'lib', 'Layout.svelte')
			}
		}),
		preprocess({
			preserve: ['ld+json'],
			scss: {
				prependData: `@import '${libPath}/scss/vars.scss';`
			}
		})
	],

	kit: {
		adapter: adapter(),
		vite: {
			build: {
				minify: true
			},
			resolve: {
				alias: {
					$assets: resolve('src', 'assets'),
					$const: resolve('src', 'const'),
					$data: resolve('src', 'data')
				}
			},
			plugins: [
				Icons({
					compiler: 'svelte'
				}),
				imagetools()
			]
		}
	}
};

export default config;