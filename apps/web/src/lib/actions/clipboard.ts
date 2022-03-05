import type { Action } from 'svelte-action-type';
import copy from 'copy-text-to-clipboard';
import { custom_event } from 'svelte/internal';

const useClipBoard: Action<HTMLElement, HTMLElement> = (node, ref) => {
	const handleClick = () => {
		const text = (ref ?? node).textContent;

		copy(text);

		const event = custom_event('copy', {
			text
		});

		node.dispatchEvent(event);
	};

	node.addEventListener('click', handleClick);

	return {
		// NOTE update function will only run if ref is a bind:this ref
		update(newRef) {
			ref = newRef;
		},
		destroy() {
			node.removeEventListener('click', handleClick);
		}
	};
};

export default useClipBoard;
