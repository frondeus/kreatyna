import type { Preview } from '@storybook/svelte';
import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
import '@skeletonlabs/skeleton/styles/skeleton.css';
import '../src/app.postcss';

const preview: Preview = {
	parameters: {
		actions: { argTypesRegex: '^on[A-Z].*' },
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/
			}
		},
		layout: 'fullscreen'
	}
};

export default preview;
