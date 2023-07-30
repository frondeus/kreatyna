<script lang="ts">
	import type { Writable } from "svelte/store";
	import { formFieldProxy } from "sveltekit-superforms/client";

    export let form;
    export let title: string;
    export let name: string;

    const { path, value, errors, constraints } = formFieldProxy(form, name);

    const boolValue = value as Writable<boolean>;
</script>

<label class="label flex items-center space-x-2">
    <input class="checkbox"
        type="checkbox"
        name={path}
        {title}
        bind:checked={$boolValue}
        aria-invalid={$errors ? 'true' : undefined}
        {...$constraints}
        />
    <p>{title}</p>
</label>
    
    