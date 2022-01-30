type InputPredicate = (val: string) => boolean;

const cantoneseRegex = '[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]';

export const hasCantonese: InputPredicate = (val) => {
	return [...val].some((char) => char.match(new RegExp(cantoneseRegex)));
};

export const isCantoneseOnly: InputPredicate = (val) => {
	// NOTE Using ^ to negate whole pattern
	// REF https://stackoverflow.com/questions/43418812/check-whether-a-string-contains-japanese-chinese-characters
	const noCantonese = `[^${cantoneseRegex.slice(1)}`;
	return !val.match(new RegExp(noCantonese));
};

export const isTraditionalOnly: InputPredicate = (val) => {
	return false;
};

const numberRegex = '\\d+';

export const hasNumber: InputPredicate = (val) => {
	return ![...val].some((char) => char.match(new RegExp(numberRegex)));
};
