/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string
  readonly VITE_WS_BASE_URL: string
  readonly VITE_APP_VERSION: string
  readonly VITE_ENVIRONMENT: string
  readonly VITE_ENABLE_DEVTOOLS: string
  readonly MODE: string
  readonly DEV: boolean
  readonly PROD: boolean
}

interface ImportMeta {
  readonly env: ImportMetaEnv
  readonly hot?: {
    accept(): void
    accept(cb: () => void): void
    accept(dep: string, cb: () => void): void
  }
}

declare const __APP_VERSION__: string
declare const __BUILD_TIME__: string

declare global {
  var __APP_VERSION__: string
  var __BUILD_TIME__: string
}
