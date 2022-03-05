type HandlerFn<T extends HTMLElement> = (elem: T) => () => void

import { replaceArabicNumber } from '$lib/transformer'

export const handleReplaceArabicNumber: HandlerFn<HTMLInputElement | HTMLTextAreaElement> = (elem) => () => {
    elem.value = replaceArabicNumber(elem.value);
    elem.dispatchEvent(new Event('change'));
}
