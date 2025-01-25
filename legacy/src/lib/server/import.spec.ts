import { importRawIngredients } from '$lib/server/import';
import { describe, it, expect } from 'vitest';
import * as fs from 'fs';

describe('import', () => {
	it.concurrent('can import english name', async () => {
		const file = fs.readFileSync('./tests/Frida_Dataset_June2023.xlsx');
		const data = await importRawIngredients(file, 'user');

		expect(data[0].name).toBe('Strawberry, raw');
	});

	it.concurrent('can import nutrients', async () => {
		const file = fs.readFileSync('./tests/Frida_Dataset_June2023.xlsx');
		const data = await importRawIngredients(file, 'user');

		expect(data[0].proteins).toBeCloseTo(0.6598, 2);
		expect(data[0].carbs).toBeCloseTo(8.3473, 2);
		expect(data[0].fat).toBeCloseTo(0.6, 2);
	});

	it.concurrent('Olive oil', async () => {
		const file = fs.readFileSync('./tests/Frida_Dataset_June2023.xlsx');
		const data = await importRawIngredients(file, 'user');

		const row = data.find((x) => x.name === 'Olive oil');
		expect(row).toBeDefined();

		expect(row?.proteins).toBeCloseTo(0, 2);
		expect(row?.carbs).toBeCloseTo(0, 2);
		expect(row?.fat).toBeCloseTo(100, 2);
	});
});
