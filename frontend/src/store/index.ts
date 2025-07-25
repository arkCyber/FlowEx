/**
 * FlowEx Redux Store Configuration
 * 
 * Enterprise-grade Redux store with persistence, middleware,
 * and comprehensive state management for the trading platform.
 */

import { configureStore, combineReducers } from '@reduxjs/toolkit'
import { persistStore, persistReducer } from 'redux-persist'
import storage from 'redux-persist/lib/storage'
import { 
  FLUSH,
  REHYDRATE,
  PAUSE,
  PERSIST,
  PURGE,
  REGISTER,
} from 'redux-persist'

// Reducers
import authReducer from './slices/authSlice'
import uiReducer from './slices/uiSlice'
import tradingReducer from './slices/tradingSlice'
import marketDataReducer from './slices/marketDataSlice'
import walletReducer from './slices/walletSlice'
import notificationReducer from './slices/notificationSlice'

// Middleware
import { rtkQueryErrorLogger } from './middleware/errorMiddleware'

// Persist configuration (not used in simplified version)
// const persistConfig = {
//   key: 'flowex-root',
//   version: 1,
//   storage,
//   whitelist: ['auth', 'ui'],
//   blacklist: ['trading', 'marketData', 'wallet', 'notifications'],
// }

// Auth persist configuration (separate for security)
const authPersistConfig = {
  key: 'flowex-auth',
  storage,
  whitelist: ['token', 'refreshToken', 'user'], // Only persist essential auth data
  blacklist: ['loading', 'error'], // Don't persist temporary state
}

// UI persist configuration
const uiPersistConfig = {
  key: 'flowex-ui',
  storage,
  whitelist: ['theme', 'sidebar'], // Persist user preferences
  blacklist: ['notifications', 'loading'], // Don't persist temporary UI state
}

// Create persisted reducers
const persistedAuthReducer = persistReducer(authPersistConfig, authReducer)
const persistedUiReducer = persistReducer(uiPersistConfig, uiReducer)

// Combined persisted reducer
const persistedRootReducer = combineReducers({
  auth: persistedAuthReducer,
  ui: persistedUiReducer,
  trading: tradingReducer,
  marketData: marketDataReducer,
  wallet: walletReducer,
  notifications: notificationReducer,
})

// Development middleware
const developmentMiddleware: any[] = []
if (import.meta.env?.DEV) {
  // Add development middleware here if needed
  console.log('Development mode enabled')
}

// Configure store
export const store = configureStore({
  reducer: persistedRootReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
        ignoredActionsPaths: ['meta.arg', 'payload.timestamp'],
        ignoredPaths: ['items.dates'],
      },
      immutableCheck: {
        warnAfter: 128,
      },
    })
      .concat(rtkQueryErrorLogger)
      .concat(developmentMiddleware),
  devTools: import.meta.env?.DEV && {
    name: 'FlowEx Trading Platform',
    trace: true,
    traceLimit: 25,
    actionSanitizer: (action: any) => ({
      ...action,
      // Sanitize sensitive data in development tools
      payload: action.type.includes('auth') && action.payload?.password
        ? { ...action.payload, password: '[REDACTED]' }
        : action.payload,
    }),
    stateSanitizer: (state: any) => ({
      ...state,
      // Sanitize sensitive data in state
      auth: {
        ...state.auth,
        token: state.auth?.token ? '[REDACTED]' : null,
        refreshToken: state.auth?.refreshToken ? '[REDACTED]' : null,
      },
    }),
  },
  enhancers: (defaultEnhancers) => {
    // Add performance monitoring in production
    if (import.meta.env?.PROD) {
      return defaultEnhancers.concat([
        // Add performance monitoring enhancer
        (createStore) => (reducer, preloadedState) => {
          const store = createStore(reducer, preloadedState)
          
          // Monitor store performance
          const originalDispatch = store.dispatch
          store.dispatch = (action) => {
            const start = performance.now()
            const result = originalDispatch(action)
            const end = performance.now()
            
            // Log slow actions
            if (end - start > 10) {
              console.warn(`Slow action detected: ${action.type} took ${end - start}ms`)
            }
            
            return result
          }
          
          return store
        },
      ])
    }
    return defaultEnhancers
  },
})

// Create persistor
export const persistor = persistStore(store)

// Export types
export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch

// Store utilities
export const getStoreState = () => store.getState()

// Action creators for common operations
export const resetStore = () => {
  persistor.purge()
  store.dispatch({ type: 'RESET_STORE' })
}

// Store health check
export const checkStoreHealth = () => {
  const state = store.getState()
  const health = {
    isHealthy: true,
    issues: [] as string[],
    timestamp: new Date().toISOString(),
  }
  
  // Check for common issues
  if (!state.auth) {
    health.isHealthy = false
    health.issues.push('Auth state is missing')
  }
  
  if (!state.ui) {
    health.isHealthy = false
    health.issues.push('UI state is missing')
  }
  
  // Check for memory leaks (large state objects)
  const stateSize = JSON.stringify(state).length
  if (stateSize > 1024 * 1024) { // 1MB
    health.isHealthy = false
    health.issues.push(`State size is too large: ${(stateSize / 1024 / 1024).toFixed(2)}MB`)
  }
  
  return health
}

// Development utilities
if (import.meta.env?.DEV) {
  // Expose store to window for debugging
  ;(window as any).__FLOWEX_STORE__ = store
  ;(window as any).__FLOWEX_PERSISTOR__ = persistor
  
  // Store health monitoring
  setInterval(() => {
    const health = checkStoreHealth()
    if (!health.isHealthy) {
      console.warn('Store health issues detected:', health.issues)
    }
  }, 30000) // Check every 30 seconds
}

// Hot module replacement for reducers
if (import.meta.hot) {
  import.meta.hot.accept('./slices/authSlice', () => {
    store.replaceReducer(persistedRootReducer)
  })
}
