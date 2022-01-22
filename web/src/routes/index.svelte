<script lang="ts" context="module">
</script>

<script lang="ts">
    const links = [{
        name: "Segment Cantonese",
        href: "/segment-cantonese"
    }, {
        name: "Cantonese to Jyutping",
        href: "/cantonese-to-jyutping"
    }]
</script>

<h1>Jyutping Tools</h1>

<p>
    This website contains tools to help you process Cantonese for learning purposes.
</p>

<ul role="list">
    {#each links as {name, href}}
        <li>
            <a {href} sveltekit:prefetch>
                {name}
            </a>
        </li>
    {/each}
</ul>

<style lang="scss">
</style>
