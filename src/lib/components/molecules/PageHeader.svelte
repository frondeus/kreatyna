<script lang="ts">
	import ActionIconButton from '$components/atoms/ActionIconButton.svelte';
	import { AppBar } from '@skeletonlabs/skeleton';
	import { createEventDispatcher } from 'svelte';
	import IconMenu from '~icons/mdi/menu';
	import IconUser from '~icons/mdi/account';
	import IconLoginVariant from '~icons/mdi/login-variant';
	import { getContext } from '$shared/context';
	import { getLinks } from '$shared/nav';

	export let isLoggedIn: boolean;

	const dispatch = createEventDispatcher();

	const page = getContext().page;
	const links = getLinks(page);

	const onHamburger = () => {
		dispatch('hamburger');
	};
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<div class="xl:hidden flex gap-2 items-center">
			{#if isLoggedIn}
				<ActionIconButton on:click={onHamburger} on:keypress={onHamburger}>
					<IconMenu />
				</ActionIconButton>
			{/if}
			<ol class="breadcrumb ml-4">
				{#each $links as { href, title, subLinks, active }}
					{#if active}
						<li class="crumb">
							<a class="anchor" {href}>
								{title}
							</a>
						</li>

						{#if subLinks}
							{#await subLinks}
								<span />
							{:then sublinks}
								<li class="crumb-separator" aria-hidden>&rsaquo;</li>
								{#each sublinks as { href, title, active }}
									{#if active}
										<li class="crumb">
											<a class="anchor" {href}>
												{title}
											</a>
										</li>
									{/if}
								{/each}
							{/await}
						{/if}
					{/if}
				{/each}
			</ol>
		</div>
	</svelte:fragment>

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
