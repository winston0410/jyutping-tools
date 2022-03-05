import { numberToChinese } from 'chinese-numbering';

type TransformerFn = (input: string) => string;

export const replaceArabicNumber: TransformerFn = (input) => {
	const isNumberRegex = /([+-]?[0-9]*[.]?[0-9]+)/;
	const mutable = input.split(isNumberRegex).filter(x => x !== "");

	let index = 0;
	for (const word of mutable) {
		if (isNumberRegex.test(word)) {
			mutable[index] = numberToChinese(parseFloat(word), {
				chineseType: 'traditional'
			});
		}

		index++;
	}
	return mutable.join('');
};

export const replaceNonCantoneseCharacters: TransformerFn = (input) => {
    
}
