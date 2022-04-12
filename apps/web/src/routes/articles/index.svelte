<script lang="ts" context="module">
    import type { Load } from '@sveltejs/kit';
    import { ARTICLES_ROUTE } from '$const'
    // import Image from '$lib/Image.svelte'
    // import max from '$lib/actions/maxHeight'
  
    export const load: Load = async ({ fetch, url }) => {
        const res = await fetch(ARTICLES_ROUTE + `?${url.searchParams.toString()}`)
        const articles = await res.json()
  
      return {
            props: {
              articles
            },
            stuff: {
                // title: "文章",
                // description: "想知點先可以喺英國生活得輕鬆啲？睇吓我哋喺英國慳錢嘅秘訣啦",
                // image: article.image,
            }
      };
  };
  </script>
  
  <script lang="ts">
     export let articles: Array<Article>
  </script>
  
  <ul role="list" class="article-list">
    {#each articles as { title, slug, description, image }}
      <li>
          <article>
            <a class="title" href={slug} sveltekit:prefetch>
              <h2>{title}</h2>
            </a>
            <a class="image" href={slug} sveltekit:prefetch>
              <Image {...image} caption={""} className={"thumbnail"}/>
            </a>
            <p>{description}</p>
          </article>
      </li>
    {/each}
  </ul>
  
  <style lang="scss">
    .article-list{
      @include grid-1-2-3;
    }
  
    //Use a javascript defined height
    .article-list article h1{
      margin-bottom: 1rem;
    }
  
    .title {
      display: block;
    }
  
    .image{
      display: block;
      margin-bottom: 1rem;
      transition: opacity var(--transition);
    }
  
    .image:hover{
      opacity: 0.75;
    }
    :global(.thumbnail){
      aspect-ratio: 16/9;
    }
    :global(.thumbnail picture, .thumbnail img){
      height: 100%;
      object-fit: fill;
    }
  </style> 