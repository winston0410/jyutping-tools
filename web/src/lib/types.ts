export enum TargetPhoneticSystem {
	Jyutping = 'jyutping',
	Yale = 'yale'
}

export type ResultTuple = [string, string];

export type ConvertResponse = {
	system_used: TargetPhoneticSystem;
	results: Array<ResultTuple>;
};

export enum InvalidCode {
	FoundArabicNumber,
	FoundNonCantoneseCharacter,
	FoundSimplifiedCharacter,
	NoCantoneseCharacter
}
