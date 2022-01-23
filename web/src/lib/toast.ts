import { toast } from '@zerodevx/svelte-toast';
import Toast from '$lib/Toast.svelte';

export const mkErrorToast = (message: string): void => {
	toast.push({
		component: {
			src: Toast,
			props: {
				message
			}
		}
	});
};
