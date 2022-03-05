import type { Action } from 'svelte-action-type';
import { custom_event } from 'svelte/internal';

const useMounted: Action<HTMLElement> = (node) => {
	const event = custom_event('mount', {});

	node.dispatchEvent(event);

	return {};
};

export default useMounted;
