import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import Icons from 'unplugin-icons/vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';

export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte'
		}),
		SvelteKitPWA({
			strategies: 'generateSW',
			srcDir: './src',
			scope: '/',
			base: '/',
			manifest: {
				short_name: 'Kreatyna',
				name: 'Kreatyna',
				description: 'Kreatyna',
				start_url: '/',
				scope: '/',
				display: 'standalone',
				theme_color: 'rgb(59 130 246 / .5)',
				background_color: 'rgb(49 58 80)',
				icons: [
					{
						src: '/pwa-192x192.png',
						sizes: '192x192',
						type: 'image/png'
					},
					{
						src: '/pwa-512x512.png',
						sizes: '512x512',
						type: 'image/png'
					},
					{
						src: '/pwa-maskable.png',
						sizes: '192x192',
						type: 'image/png',
						purpose: 'maskable'
					}
				]
			},
			devOptions: {
				enabled: true,
				suppressWarnings: process.env.SUPPRESS_WARNINGS === 'true',
				type: 'module',
				navigateFallback: '/'
			},
			kit: {
				includeVersionFile: true
			}
		})
	],
	test: {
		testTimeout: 10000,
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	server: {
		fs: {
			allow: ['prisma/generated']
		}
	}
});
