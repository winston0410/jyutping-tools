<script lang="ts" context="module">
    import 'modern-css-reset'
</script>

<script lang="ts">
</script>

<div>
    <slot />
</div>
<footer>
    <p>Hello world</p>
</footer>

<style lang="scss">
    :global(body){
        background: hsla(46, 75%, 87%, 1);
    }

    :global(button){
        appearance: none;
        margin: 0;
        padding: 0;
        border: 0;
        cursor: pointer;
    }

    footer{
        background: $white;
        padding: 12px;
    }
</style>
