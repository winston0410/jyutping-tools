export enum TargetPhoneticSystem {
	Jyutping = 'jyutping',
	ToneNumberYale = 'yale(tone number)',
	ToneMarkYale = 'yale(tone mark)'
}

export type ResultTuple = [string, string];

export type ConvertResponse = {
	system_used: TargetPhoneticSystem;
	results: Array<ResultTuple>;
};

// NOTE Cannot use 0 as the beginning number, as it is falsy and Zod will treat it as non-existent
export enum InvalidCode {
	FoundArabicNumber = 1,
	FoundNonCantoneseCharacter = 2,
	FoundSimplifiedCharacter = 3,
	NoCantoneseCharacter = 4,
	EmptySelfInput = 5
}
