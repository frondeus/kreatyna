<script lang="ts">
	import { onMount } from "svelte";

    onMount(() => {
        document.getElementsByTagName( 'html' )[0]
            .setAttribute( 'class', 'dark' );
    });
</script>

<div style="display: contents" class="h-full overflow-hidden">
    <slot />
</div>