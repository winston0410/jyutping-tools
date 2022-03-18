import env from '$lib/env';

export const MICROSERVICE_ROOT = env.VITE_MICROSERVICE_URL;

export const PROXY_ROOT = '/api';

export const PARSE_ACTION = '/parse';

//  export const MICROSERVICE_CREDENTIAL =
//  'Basic ' + btoa(`${env.VITE_MICROSERVICE_USER}:${env.VITE_MICROSERVICE_PWD}`);
