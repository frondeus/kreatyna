<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="card variant-ghost-error">
	<header class="card-header">
		<h1 class="h1">Oops!</h1>
	</header>
	<section class="p-4">
		<p>{$page.error?.message}</p>
	</section>
	<footer class="card-footer">
		<a href="/browse/raw" class="btn variant-filled-primary">Return to list</a>
	</footer>
</div>
