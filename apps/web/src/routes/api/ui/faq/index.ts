import type { RequestHandler } from '@sveltejs/kit';

export const get: RequestHandler = async () => {
	return {
		body: [
			{
				question: 'What technologies power this Cantonese to romanization converter?',
				answer:
					'This converter is built with a custom Cantonese NLP engine written in Rust, and we use the datasets from <a class="link" rel="external" href="https://words.hk/">words.hk</a>. We do not have any Python dependencies like <a class="link" rel="external" href="https://pypi.org/project/pycantonese/">PyCantonese</a>, as it is too slow for handling large amount of text.'
			},
			{
				question: 'What romanization can I convert Cantonese characters to with this converter?',
				answer:
					'You can convert Cantonese characters to Jyutping, Yale with tone marks or Yale with tone numbers with this converter.'
			},
			{
				question: 'Why would I get "unknown" as the output for a certain character?',
				answer:
					'If you are getting "unknown" as the output, it means that characters is not included in our datasets. You can <a class="link" rel="external" href="https://github.com/winston0410/jyut.info/issues/new?assignees=&labels=romanization&template=romanization_report.md&title=">submit an issue</a> for us to improve our converter.'
			}
		]
	};
};
