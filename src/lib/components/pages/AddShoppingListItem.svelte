<script lang="ts">
	import ActionIconButton from '$components/atoms/buttons/ActionIconButton.svelte';
	import FloatingButton from '$components/atoms/buttons/FloatingButton.svelte';
	import IconArrowLeft from '~icons/mdi/arrow-left';
	import IconCheck from '~icons/mdi/check';
	import ShoppingListItemProposal, {
		type Proposal
	} from '$components/molecules/ShoppingListItemProposal.svelte';
	import { v4 as uuid } from 'uuid';
	import { createEventDispatcher } from 'svelte';

	export let proposals: Proposal[];

	let searchInput = '';
	let changed = false;
	const dispatch = createEventDispatcher();
	$: trimmedSearchInput = searchInput.trim();

	let suggestion: Proposal | undefined = undefined;
	$: {
		if (trimmedSearchInput.length > 0 && !proposals.some((p) => p.name === trimmedSearchInput)) {
			suggestion = { name: trimmedSearchInput, amount: 0, id: uuid(), state: 'TODO' };
		} else {
			suggestion = undefined;
		}
	}
	const onAdd = (event: CustomEvent<Proposal>) => {
		const proposal = event.detail;
		if (!proposals.some((p) => p.id === proposal.id)) {
			proposals = [proposal, ...proposals];
			changed = true;
		}
		searchInput = '';
	};

	const onSubmit = () => {
		if (suggestion) {
			suggestion.amount += 1;
			proposals = [suggestion, ...proposals];
			suggestion = undefined;
			searchInput = '';
		}
		dispatch('submit', proposals);
	};

	const onBack = () => {
		dispatch('submit', proposals);
	};
</script>

<div class="h-full">
	<header class="p-2 bg-surface-800 flex gap-2">
		<ActionIconButton on:click={onBack} on:keypress={onBack}>
			<IconArrowLeft />
		</ActionIconButton>
		<!-- svelte-ignore a11y-autofocus -->
		<input
			class="input"
			type="text"
			placeholder="Add new item"
			enterkeyhint="search"
			on:keyup={(e) => e.key === 'Enter' && onSubmit()}
			bind:value={searchInput}
			autofocus
		/>
	</header>

	<ul class="space-y-2 p-2">
		{#if suggestion}
			<ShoppingListItemProposal
				name={suggestion.name}
				bind:amount={suggestion.amount}
				bind:state={suggestion.state}
				id={suggestion.id}
				on:add={onAdd}
			/>
		{/if}
		{#each proposals as { name, amount, id, state } (id)}
			<ShoppingListItemProposal {name} bind:amount bind:state {id} on:add={onAdd} />
		{/each}
	</ul>

	{#if changed || trimmedSearchInput.length > 0}
		<FloatingButton on:click={onSubmit} on:keypress={onSubmit}>
			<IconCheck />
		</FloatingButton>
	{/if}
</div>
