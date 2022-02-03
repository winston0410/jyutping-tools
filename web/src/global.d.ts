/// <reference types="@sveltejs/kit" />
//
import type { InvalidCode } from '$lib/types'

declare module "zod" {
    export type CustomErrorParams = InvalidCode
}
