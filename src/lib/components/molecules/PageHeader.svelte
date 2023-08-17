<script lang="ts">
	import ActionIconButton from "$components/atoms/ActionIconButton.svelte";
	import { AppBar } from "@skeletonlabs/skeleton";
	import { createEventDispatcher } from "svelte";
    import IconMenu from '~icons/mdi/menu';
    import IconUser from '~icons/mdi/account';
    import IconLoginVariant from '~icons/mdi/login-variant';

    export let isLoggedIn: boolean;

    const dispatch = createEventDispatcher();

    const onHamburger = () => {
        dispatch('hamburger');
    };
</script>


<AppBar gridColumns="grid-cols-3"
        slotDefault="place-self-center"
        slotTrail="place-content-end">
	<svelte:fragment slot="lead">
        <div class="lg:hidden">
            {#if isLoggedIn}
            <ActionIconButton on:click={onHamburger} on:keypress={onHamburger}>
                <IconMenu />
            </ActionIconButton>
            {/if}
        </div>
    </svelte:fragment>

    Kreatyna

	<svelte:fragment slot="trail">
    	{#if isLoggedIn}
            <ActionIconButton href="/account">
                <IconUser />
            </ActionIconButton>
    	{:else}
            <ActionIconButton href="/login">
                <IconLoginVariant />
            </ActionIconButton>
    		<!-- <NavAnchor href="/login" title="Login">
    			Login
    		</NavAnchor> -->
    	{/if}
    </svelte:fragment>
</AppBar>


