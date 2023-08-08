import type { PrismaClient } from '@prisma/client';
import type { RecipeMap, RecipeSummaryMap } from './map';
import type { RecipySummary } from '$lib/shared/domains/recipe/models';

export class RecipeService {
	map: RecipeMap;
	summaryMap: RecipeSummaryMap;
	prisma: PrismaClient;

	constructor(map: RecipeMap, summaryMap: RecipeSummaryMap, prisma: PrismaClient) {
		this.map = map;
		this.prisma = prisma;
		this.summaryMap = summaryMap;
	}
	async getAll(): Promise<RecipySummary[]> {
		const summaries = await this.prisma.recipie.findMany({
			select: { id: true, name: true }
		});
		return summaries.map((recipe) => this.summaryMap.toDomain(recipe));
	}
	// async get(id: number) {
	// 	const recipe = await this.prisma.recipie.findUnique({
	// 		where: {
	// 			id
	// 		},
	// 		include: {
	// 			ingredients: {
	// 				include: {
	// 					raw: true
	// 				}
	// 			}
	// 		}
	// 	});
	// 	return this.map.toDomain(recipe);
	// }
	// async create(recipe: Recipy) {
	// 	const dbEntry = await this.prisma.recipie.create({
	// 		data: this.map.toDB(recipe)
	// 	});
	// 	return this.map.toDomain(dbEntry);
	// }
	// async update(recipe: Recipy) {
	// 	const dbEntry = await this.prisma.recipie.update({
	// 		where: {
	// 			id: recipe.id
	// 		},
	// 		data: this.map.toDB(recipe)
	// 	});
	// 	return this.map.toDomain(dbEntry);
	// }
	// async remove(id: number) {
	// 	await this.prisma.recipie.delete({
	// 		where: {
	// 			id
	// 		}
	// 	});
	// }
}
