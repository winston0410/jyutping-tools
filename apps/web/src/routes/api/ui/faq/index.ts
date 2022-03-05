import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async () => {
	return {
		body: [
			{
				question: 'What technologies power this Cantonese to romanization converter?',
				answer:
					'This converter is built with a custom Cantonese NLP engine written in Rust, and we use the dataset from words.hk. We do not have any Python dependencies, as it is too slow for handling large amount of text.'
			},
			{
				question: 'What romanization can I convert Cantonese characters to with this converter?',
				answer:
					'You can convert Cantonese characters to Jyutping, Yale with tone marks or Yale with tone numbers with this converter.'
			}
		]
	};
};
