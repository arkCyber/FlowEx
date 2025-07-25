/**
 * FlowEx Performance Monitoring Utilities
 */

export const initializePerformanceMonitoring = () => {
  if (typeof window === 'undefined') return

  // Web Vitals monitoring
  if ('PerformanceObserver' in window) {
    // Largest Contentful Paint
    const lcpObserver = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        console.log('LCP:', entry.startTime)
      }
    })
    lcpObserver.observe({ entryTypes: ['largest-contentful-paint'] })

    // First Input Delay
    const fidObserver = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        console.log('FID:', (entry as any).processingStart - entry.startTime)
      }
    })
    fidObserver.observe({ entryTypes: ['first-input'] })

    // Cumulative Layout Shift
    const clsObserver = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        if (!(entry as any).hadRecentInput) {
          console.log('CLS:', (entry as any).value)
        }
      }
    })
    clsObserver.observe({ entryTypes: ['layout-shift'] })
  }

  // Memory usage monitoring
  if ('memory' in performance) {
    setInterval(() => {
      const memory = (performance as any).memory
      if (memory.usedJSHeapSize > 50 * 1024 * 1024) { // 50MB
        console.warn('High memory usage detected:', memory.usedJSHeapSize / 1024 / 1024, 'MB')
      }
    }, 30000)
  }
}
