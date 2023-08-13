<script lang="ts">
	interface ColumnDefinition {
		name: string;
	}

	export let columns: ColumnDefinition[];
</script>

<div class="table-container">
	<table class="table max-md:table-compact">
		<thead>
			<tr>
				{#each columns as column}
					<th>{column.name}</th>
				{/each}
				<th class="flex justify-end">
					<slot name="headerActions" />
				</th>
			</tr>
		</thead>
		<tbody>
			<slot />
		</tbody>
	</table>
</div>
