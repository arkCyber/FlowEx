/**
 * FlowEx Test Setup
 */

import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Mock environment variables
Object.defineProperty(window, '__FLOWEX_CONFIG__', {
  value: {
    API_BASE_URL: 'http://localhost:8001',
    WS_BASE_URL: 'ws://localhost:8001',
    APP_VERSION: '1.0.0',
    BUILD_TIME: '2024-01-01T00:00:00Z',
    ENVIRONMENT: 'test',
  },
  writable: true,
})

// Mock global constants
;(global as any).__APP_VERSION__ = '1.0.0'
;(global as any).__BUILD_TIME__ = '2024-01-01T00:00:00Z'

// Mock IntersectionObserver
global.IntersectionObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}))

// Mock matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
})

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
  length: 0,
  key: vi.fn(),
}
;(global as any).localStorage = localStorageMock

// Mock sessionStorage
const sessionStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
  length: 0,
  key: vi.fn(),
}
;(global as any).sessionStorage = sessionStorageMock
