import type { Action } from 'svelte-action-type';
import copy from 'copy-text-to-clipboard';

const useClipBoard: Action<HTMLElement, HTMLElement> = (node, ref) => {
	const handleClick = () => {
		console.log('copying', node, ref);
	};

	node.addEventListener('click', handleClick);

	return {
		destroy() {
			node.removeEventListener('click', handleClick);
		}
	};
};

export default useClipBoard;
