<script lang="ts">
	import BottomBar from "$components/molecules/BottomBar.svelte";
	import Sidebar from "$components/molecules/Sidebar.svelte";
	import { AppShell, Modal, Toast } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    onMount(() => {
        var root = document.getElementsByTagName( 'html' )[0];
        root.setAttribute( 'class', 'dark' );
    });
</script>

<div style="display: contents" class="h-full overflow-hidden">
    <AppShell slotSidebarLeft="w-0 lg:w-64" slotFooter="lg:hidden">
    	<svelte:fragment slot="sidebarLeft">
    		<Sidebar isLoggedIn={true} />
    	</svelte:fragment>
    	<div class="lg:mx-auto lg:p-8 lg:space-y-8">
    		Test
    	</div>
    	<svelte:fragment slot="footer">
    		<BottomBar />
    	</svelte:fragment>
    </AppShell>
    <Modal />
    <Toast />
</div>
