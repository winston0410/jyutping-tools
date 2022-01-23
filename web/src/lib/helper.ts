export const isCantoneseOnly = (val: string): boolean => {
    // NOTE Using ^ to negate whole pattern
    // REF https://stackoverflow.com/questions/43418812/check-whether-a-string-contains-japanese-chinese-characters
	const chineseRegex = /[^\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f]/g;
	return !val.match(chineseRegex);
};

export const isTraditionalOnly = (val: string): boolean => {
	return false;
};
