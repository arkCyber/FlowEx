/**
 * FlowEx Error Reporting Utilities
 */

export const initializeErrorReporting = () => {
  if (typeof window === 'undefined') return

  // Global error handler
  window.addEventListener('error', (event) => {
    console.error('Global error:', event.error)
    reportError(event.error, 'global')
  })

  // Unhandled promise rejection handler
  window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason)
    reportError(event.reason, 'promise')
  })
}

const reportError = (error: any, type: string) => {
  if (import.meta.env.PROD) {
    // Report to error tracking service
    fetch('/api/errors', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        message: error.message,
        stack: error.stack,
        type,
        userAgent: navigator.userAgent,
        url: window.location.href,
        timestamp: new Date().toISOString(),
      }),
    }).catch(console.error)
  }
}
