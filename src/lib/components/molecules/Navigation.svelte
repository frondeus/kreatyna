<script lang="ts">
	import { getContext } from '$shared/context';
	import { getLinks } from '$shared/nav';

	const page = getContext().page;
	const links = getLinks(page);
</script>

<nav class="list-nav">
	<ul>
		{#each $links as { href, title, subLinks, active }}
			<li>
				<a class:bg-primary-500={active} {href} {title}>{title}</a>
			</li>
			{#if subLinks}
				{#await subLinks}
					<li class="pl-4">Loading...</li>
				{:then sublinks}
					{#each sublinks as { href: subHref, title, active: subActive }}
						<li class="pl-4">
							<a href={subHref} class:bg-surface-800={!subActive} class:bg-primary-500={subActive}
								>{title}</a
							>
						</li>
					{/each}
				{/await}
			{/if}
		{/each}
	</ul>
</nav>
