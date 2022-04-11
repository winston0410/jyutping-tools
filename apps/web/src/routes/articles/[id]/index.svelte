<script lang="ts" context="module">
	import type { Load } from '@sveltejs/kit';
	import { ARTICLES_ROUTE } from '$const'
	export const load: Load = async ({ fetch, url, params }) => {
        try {
        const res = await fetch(ARTICLES_ROUTE)
        const body = await res.json()
        const article = body.find((elem) => elem.slug === `/articles/${params.id}`)
        if(!article){
            return {
                status: 404
            }
        }
        const content = await import(`../../../data/${article.fileName}.svelte.md`)
		return {
            props: {
                content: content.default
            },
            stuff: {
                title: article.title,
		        description: article.description,
		        image: article.image,
		        keywords: article.keywords,
                createdAt: article.created_at
            }
		};
        } catch (error) {
            console.error(error)
            return {
                status: 500
            }
        }
	};
</script>

<script lang="ts">
    // TODO Fix the typing for svelte object later
    export let content: unknown;
</script>

<svelte:component this={content} />