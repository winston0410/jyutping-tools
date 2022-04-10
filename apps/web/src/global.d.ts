/// <reference types="@sveltejs/kit" />
type Theme = 'light' | 'dark';

type FAQEntity = {
	question: string;
	answer: string;
};

type ArticleCover = {
	src: string;
	srcset: string;
	alt: string;
	format: string;
	caption: string;
	width: number;
	height: number;
};

type Article = {
	title: string;
	description: string;
	// Has trouble for compiling and building blog post file in UTF8. Therefore to need include the fileName field
	slug: string;
	// This will contain the fileName of the target post, and it will be used as a reference
	fileName: string;
	image: ArticleCover;
	updated_at: string;
	created_at: string;
	keywords: Array<string>;
};
