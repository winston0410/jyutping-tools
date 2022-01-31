import type { ResultTuple } from '$lib/types';
type FormatFn = (input: any) => string;

type ExtractPhonetic = (results: Array<ResultTuple>) => Array<string>;

export const extractPhonetic: ExtractPhonetic = (results) =>
	results.map(([, phonetic]) => {
		return phonetic;
	});

export const formatPhonetic: FormatFn = (input: any) => {
	console.log(input);
	return input;
};
