import { writable } from 'svelte/store';
import type { Readable } from 'svelte/store';
import type { Action } from 'svelte-action-type';

type Enum = { [key: string]: unknown };

//  Using valueOf
// REF https://stackoverflow.com/questions/49285864/is-there-a-valueof-similar-to-keyof-in-typescript
type ValueOf<T> = T[keyof T];

type MkTabsArgs<T extends Enum> = {
	initial: ValueOf<T>;
};

interface TabsStore<T extends Enum> {
	currentTab: Readable<ValueOf<T>>;
	tab: Action<HTMLElement, ValueOf<T>>;
}

const mkTabs = <T extends Enum>(opts: MkTabsArgs<T>): TabsStore<T> => {
	const { subscribe, set } = writable(opts.initial);

	return {
		currentTab: { subscribe },
		// NOTE Action to use with element
		tab: (node, tabName) => {
			const handleClick = () => {
				set(tabName);
			};

			node.addEventListener('click', handleClick);

			return {
				destroy() {
					node.removeEventListener('click', handleClick);
				}
			};
		}
	};
};

export default mkTabs;
