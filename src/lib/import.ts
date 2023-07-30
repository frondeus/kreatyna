import type { RawIngredientInsert } from './types';
import { ImporterFactory } from 'xlsx-import/lib/ImporterFactory';

export async function importRawIngredients(
	data: Buffer,
	created_by: string
): Promise<RawIngredientInsert[]> {
	const config = {
		raw: {
			type: 'list',
			worksheet: 'Data_Table',
			columns: [
				{
					index: 2, // B
					key: 'name',
					mapper: (value: string) => value.trim()
				},
				{
					index: 6, // F
					key: 'proteins',
					mapper: (value: string) => parseFloat(value.replace(',', '.')) || 0
				},
				{
					index: 9, // I
					key: 'carbs',
					mapper: (value: string) => parseFloat(value.replace(',', '.')) || 0
				},
				{
					index: 13, // M
					key: 'fat',
					mapper: (value: string) => parseFloat(value.replace(',', '.')) || 0
				}
			],
			rowOffset: 4
		}
	};

	const factory = new ImporterFactory();

	const importer = await factory.fromBuffer(data);

	const raw = importer.getAllItems<RawIngredientInsert>(config.raw);
	return raw.map((r) => ({ ...r, created_by, container: 100 }));
}
