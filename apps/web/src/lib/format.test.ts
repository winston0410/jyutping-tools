import { extractPhonetic } from './format';
import type { ResultTuple } from '$lib/types';

describe('extractPhonetic', () => {
	describe('when given unknown phonetic', () => {
		const data: Array<ResultTuple> = [['沊', ['unknown']]];

		it('suffix the word with the unknown', () => {
			const formatted = extractPhonetic(data);

			expect(formatted).toStrictEqual(['unknown(沊)']);
		});
	});
});
