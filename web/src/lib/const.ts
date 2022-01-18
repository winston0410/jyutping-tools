import env from '$lib/env'

const MICROSERVICE_VERSION = "/v1"

export const MICROSERVICE_ROOT = env.VITE_MICROSERVICE_URL + MICROSERVICE_VERSION

export const PROXY_ROOT = "/api"

export const CONVERT_ACTION = "/convert"
