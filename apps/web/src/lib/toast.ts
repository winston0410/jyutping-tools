import type { addNotification, DefaultNotificationOptions } from 'svelte-notifications';

type MkToast = (text: string) => void;

type Toaster = {
	mkError: MkToast;
	mkWarning: MkToast;
	mkOk: MkToast;
};

export enum ToastKind {
	Error = 'error',
	Warning = 'warning',
	Info = 'info',
	Ok = 'ok'
}

export interface Toast extends Omit<DefaultNotificationOptions, 'type'> {
	kind: ToastKind;
	// TODO Introduce icon to toast
	//  icon: string;
}

const position = 'bottom-center';

const mkToaster = (add: addNotification): Toaster => {
	return {
		mkError: (text) => {
			add({
				text,
				kind: ToastKind.Error,
				position,
				removeAfter: 3000
			});
		},
		mkWarning: (text) => {
			add({
				text,
				kind: ToastKind.Warning,
				position,
				removeAfter: 3000
			});
		},
		mkOk: (text) => {
			add({
				text,
				kind: ToastKind.Ok,
				position,
				removeAfter: 3000
			});
		}
	};
};

export default mkToaster;
