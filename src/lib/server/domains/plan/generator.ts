import { Lifecycle, inject, injectable, scoped } from 'tsyringe';
import type { TagService } from '../tag/service';
import type { WeekPlanCreate, WeekPlanMealWithoutId } from '$shared/domains/plan/models';

@scoped(Lifecycle.ContainerScoped)
@injectable()
export class WeekPlanGenerator {
	constructor(@inject('tagService') readonly tagService: TagService) {}

	async generateWeekPlan(year: number, week: number): Promise<WeekPlanCreate> {
		const forTag = async (tag: string) =>
			await this.tagService.find({ category: 'for', name: tag });

		const [forBreakfast, forSnack, forLunch, forDesert, forDinner] = await Promise.all(
			['breakfast', 'snack', 'lunch', 'desert', 'dinner'].map(forTag)
		);
		const dayIndices = [...Array(7).keys()].map((day) => day + 1);
		const meals: WeekPlanMealWithoutId[] = [
			...dayIndices.map((day) => ({
				day,
				slot: 2,
				size: 2,
				tags: [...forBreakfast],
				name: 'Breakfast',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 6,
				size: 1,
				tags: [...forSnack],
				name: 'Snack',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 10,
				size: 2,
				tags: [...forLunch],
				name: 'Lunch',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 14,
				size: 1,
				tags: [...forDesert],
				name: 'Desert',
				recipe: null
			})),
			...dayIndices.map((day) => ({
				day,
				slot: 17,
				size: 2,
				tags: [...forDinner],
				name: 'Dinner',
				recipe: null
			}))
		];

		return {
			year,
			week,
			meals
		};
	}
}
