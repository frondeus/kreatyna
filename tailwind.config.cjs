/** @type {import('tailwindcss').Config}*/

// Creates an object with pattern 'number': 'number'
const gridRowStart = Object.fromEntries(
	Array.from({ length: 18 }, (_, i) => [i + 1, (i + 1).toString()])
);

const flex = Object.fromEntries(
	Array.from({ length: 6 }, (_, i) => [i + 1, `${i + 1} ${i + 1} 0%`])
);

const gridTemplateRows = Object.fromEntries(
	Array.from({ length: 18 }, (_, i) => [i + 1, `repeat(${i + 1}, minmax(0, 1fr))`])
);

console.log(gridTemplateRows);

const config = {
	darkMode: 'class',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		require('path').join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
	],
	theme: {
		extend: {
			gridRowStart,
			flex,
			gridTemplateRows
		}
	},
	safelist: [
		{
			pattern: /variant-glass-.*/
		},
		{
			pattern: /variant-ringed-.*/
		},
		{
			pattern: /col-start-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		},
		{
			pattern: /col-span-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		},
		{
			pattern: /row-span-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		},
		{
			pattern: /row-start-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		},
		{
			pattern: /flex-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		},
		{
			pattern: /grid-rows-\d+/,
			variants: ['sm', 'md', 'lg', 'xl', '2xl']
		}
	],

	plugins: [
		require('@tailwindcss/forms'),
		...require('@skeletonlabs/skeleton/tailwind/skeleton.cjs')()
	]
};

module.exports = config;
