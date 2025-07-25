/**
 * FlowEx Frontend Application Entry Point
 * 
 * Enterprise-grade React application with comprehensive error handling,
 * performance monitoring, and production-ready configuration.
 */

import React from 'react'
import ReactDOM from 'react-dom/client'
import { Provider } from 'react-redux'
import { PersistGate } from 'redux-persist/integration/react'
import { BrowserRouter } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
// DevTools only in development
import { HelmetProvider } from 'react-helmet-async'
import { ErrorBoundary } from 'react-error-boundary'
import { Toaster } from 'react-hot-toast'

import App from './App'
import { store, persistor } from './store'
import { ErrorFallback } from './components/ErrorBoundary'
import { LoadingSpinner } from './components/LoadingSpinner'
import { registerSW } from './utils/serviceWorker'

// Import global styles
import './styles/global.css'

// Performance monitoring
import { initializePerformanceMonitoring } from './utils/performance'

// Error reporting
import { initializeErrorReporting } from './utils/errorReporting'

// Initialize performance monitoring
initializePerformanceMonitoring()

// Initialize error reporting
initializeErrorReporting()

// Create React Query client with enterprise configuration
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
      retry: (failureCount, error: any) => {
        // Don't retry on 4xx errors except 408, 429
        if (error?.response?.status >= 400 && error?.response?.status < 500) {
          if (error.response.status === 408 || error.response.status === 429) {
            return failureCount < 2
          }
          return false
        }
        // Retry up to 3 times for other errors
        return failureCount < 3
      },
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
    },
    mutations: {
      retry: 1,
      retryDelay: 1000,
    },
  },
})

// Global error handler
const handleError = (error: Error, errorInfo: { componentStack?: string | null }) => {
  console.error('Application Error:', error)
  console.error('Component Stack:', errorInfo.componentStack)
  
  // Report to error tracking service
  if (window.gtag) {
    window.gtag('event', 'exception', {
      description: error.message,
      fatal: false,
    })
  }
  
  // Report to custom error service
  if (import.meta.env.PROD) {
    fetch('/api/errors', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        message: error.message,
        stack: error.stack,
        componentStack: errorInfo?.componentStack || '',
        userAgent: navigator.userAgent,
        url: window.location.href,
        timestamp: new Date().toISOString(),
      }),
    }).catch(console.error)
  }
}

// Register service worker for PWA functionality
if ('serviceWorker' in navigator && import.meta.env.PROD) {
  registerSW()
}

// Performance observer for monitoring
if ('PerformanceObserver' in window) {
  const observer = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      if (entry.entryType === 'navigation') {
        console.log('Navigation timing:', entry)
      }
      if (entry.entryType === 'largest-contentful-paint') {
        console.log('LCP:', entry.startTime)
      }
      if (entry.entryType === 'first-input') {
        console.log('FID:', (entry as any).processingStart - entry.startTime)
      }
    }
  })
  
  observer.observe({ entryTypes: ['navigation', 'largest-contentful-paint', 'first-input'] })
}

// Main application component
const FlowExApp: React.FC = () => {
  return (
    <ErrorBoundary
      FallbackComponent={ErrorFallback}
      onError={handleError}
      onReset={() => window.location.reload()}
    >
      <HelmetProvider>
        <Provider store={store}>
          <PersistGate loading={<LoadingSpinner />} persistor={persistor}>
            <QueryClientProvider client={queryClient}>
              <BrowserRouter>
                  <App />
                  <Toaster
                    position="top-right"
                    toastOptions={{
                      duration: 4000,
                      style: {
                        background: '#363636',
                        color: '#fff',
                      },
                      success: {
                        duration: 3000,
                        iconTheme: {
                          primary: '#4ade80',
                          secondary: '#fff',
                        },
                      },
                      error: {
                        duration: 5000,
                        iconTheme: {
                          primary: '#ef4444',
                          secondary: '#fff',
                        },
                      },
                    }}
                  />
                </BrowserRouter>

              {/* DevTools only in development */}
            </QueryClientProvider>
          </PersistGate>
        </Provider>
      </HelmetProvider>
    </ErrorBoundary>
  )
}

// Render application
const rootElement = document.getElementById('root')
if (!rootElement) {
  throw new Error('Root element not found')
}

const root = ReactDOM.createRoot(rootElement)

// Render with React 18 concurrent features
root.render(<FlowExApp />)

// Hot module replacement for development
if (import.meta.hot) {
  import.meta.hot.accept()
}

// Log application startup
console.log(`
🚀 FlowEx Frontend Application Started
Version: ${import.meta.env.VITE_APP_VERSION || '1.0.0'}
Environment: ${import.meta.env.MODE}
`)

// Export for testing
export { queryClient }
