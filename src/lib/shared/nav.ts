import type { Page } from '@sveltejs/kit';
import type { WeekPlanSummary } from './domains/plan/models';
import { derived, type Readable } from 'svelte/store';

export interface LinkDefinition {
	href: string;
	title: string;
	onActive?: () => Promise<LinkDefinition[]>;
}

interface Link {
	href: string;
	title: string;
	active: boolean;
	subLinks?: Promise<Link[]>;
}

export const getLinks: (page: Readable<Page>) => Readable<Link[]> = ($page) => {
	const defs = derived($page, ($page) => [
		{
			href: '/plan',
			title: 'Plan',
			onActive: async () => {
				const current = $page.data.current;
				const isCurrent = (plan: WeekPlanSummary) =>
					plan.week === current.week && plan.year === current.year;
				return $page.data.plans.map((plan: WeekPlanSummary) => ({
					title: 'Week ' + plan.week,
					href: isCurrent(plan) ? '/plan' : '/plan/' + plan.year + '/' + plan.week
				}));
			}
		},
		{
			href: '/browse',
			title: 'Browse',
			onActive: async () => {
				return [
					{ href: '/browse/recipe', title: 'Recipes' },
					{ href: '/browse/raw', title: 'Raw ingredients' },
					{ href: '/browse/tag', title: 'Tags' }
				];
			}
		}
	]);

	const links = derived([defs, $page], ([$defs, $page]) => {
		const isActive = (href: string) => $page.url.pathname.startsWith(href);
		const isActiveEx = (href: string, parent: string) => isActive(href) && parent !== href;
		return $defs.map((def) => ({
			href: def.href,
			title: def.title,
			active: isActive(def.href),
			subLinks:
				isActive(def.href) && def.onActive
					? def.onActive().then((sublinks) => {
							console.log({ url: $page.url.pathname, sublinks, def: def.href });
							return sublinks.map((sublink: LinkDefinition) => ({
								href: sublink.href,
								title: sublink.title,
								active: isActiveEx(sublink.href, def.href)
							}));
					  })
					: undefined
		}));
	});

	return links;
};

// {
// 	href: '/shop',
// 	title: 'Shop',
// },
// {
// 	href: '/store',
// 	title: 'Store',
// },
// {
// 	href: '/cook',
// 	title: 'Cook',
// },
// {
// 	href: '/log',
// 	title: 'Log',
// },
