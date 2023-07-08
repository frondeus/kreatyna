<script lang="ts">
	import { Tab, TabGroup } from "@skeletonlabs/skeleton";
  import { popup } from '@skeletonlabs/skeleton';
  import type { PopupSettings } from '@skeletonlabs/skeleton';
	import BottomAnchor from "./BottomAnchor.svelte";

  export let isLoggedIn: boolean;
  let group = 0;


  const popupHamburger: PopupSettings = {
  	event: 'click',
  	target: 'popupHamburger',
  	placement: 'top',
  	closeQuery: '.close-burger'
  };
					
</script>

<TabGroup 
	justify="justify-center"
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	flex="flex-1 lg:flex-none"
	rounded=""
	border=""
	class="bg-surface-100-800-token w-full"
>
	<BottomAnchor href="/plan" >Pl</BottomAnchor>
	<BottomAnchor href="/shop" >Sh</BottomAnchor>
	<BottomAnchor href="/store">St</BottomAnchor>
	<BottomAnchor href="/cook" >Co</BottomAnchor>
  
  <div class="card p-4 max-w-sm" data-popup="popupHamburger">
  	<div class="grid grid-cols-1 gap-2">
  		<a href="/" class="btn close-burger">Kreatyna</a>
  		<a href="/log" class="btn close-burger">Log</a>
  		<a href="/browse" class="btn close-burger">Browse</a>
  	</div>
  </div>
					
  <button class="btn" use:popup={popupHamburger}>...</button>
	<!-- 
		{#if isLoggedIn}
			<BottomAnchor href="/account">
				Account
			</BottomAnchor>
		{:else}
			<BottomAnchor href="/login">
				Login
			</BottomAnchor>
		{/if}
   -->
</TabGroup>
