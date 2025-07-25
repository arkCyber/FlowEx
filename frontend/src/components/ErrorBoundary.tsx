/**
 * FlowEx Error Boundary Component with Tailwind CSS
 */

import React from 'react'
import { AlertTriangle, RefreshCw, Home } from 'lucide-react'
import { cn } from '../utils/cn'

interface ErrorFallbackProps {
  error: Error
  resetErrorBoundary: () => void
}

export const ErrorFallback: React.FC<ErrorFallbackProps> = ({
  error,
  resetErrorBoundary,
}) => {
  const goHome = () => {
    window.location.href = '/dashboard'
  }

  return (
    <div className="min-h-screen bg-background-warm flex items-center justify-center p-4">
      <div className="max-w-md w-full text-center">
        <div className="card p-8">
          {/* Error Icon */}
          <div className="flex justify-center mb-6">
            <div className="w-16 h-16 bg-red-100 dark:bg-red-900/20 rounded-full flex items-center justify-center">
              <AlertTriangle size={32} className="text-red-600 dark:text-red-400" />
            </div>
          </div>

          {/* Error Title */}
          <h1 className="text-2xl font-bold text-white mb-4">
            Something went wrong
          </h1>

          {/* Error Description */}
          <p className="text-warm-400 mb-6">
            We're sorry, but something unexpected happened. Please try refreshing the page or go back to the dashboard.
          </p>

          {/* Error Details (Development Only) */}
          {import.meta.env.DEV && (
            <div className="mb-6 p-4 bg-background-warm-tertiary rounded-lg border border-warm-700">
              <h3 className="text-sm font-medium text-warm-300 mb-2">Error Details:</h3>
              <pre className="text-xs text-red-400 text-left overflow-auto max-h-32">
                {error.message}
              </pre>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex flex-col sm:flex-row gap-3 justify-center">
            <button
              onClick={resetErrorBoundary}
              className="btn-primary flex items-center justify-center gap-2"
            >
              <RefreshCw size={16} />
              Try Again
            </button>

            <button
              onClick={goHome}
              className="btn-outline flex items-center justify-center gap-2"
            >
              <Home size={16} />
              Go Home
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

/**
 * Compact Error Component for smaller spaces
 */
export const CompactError: React.FC<{
  message?: string
  onRetry?: () => void
  className?: string
}> = ({
  message = 'Something went wrong',
  onRetry,
  className
}) => {
  return (
    <div className={cn(
      'flex flex-col items-center justify-center p-6 text-center',
      className
    )}>
      <AlertTriangle size={24} className="text-red-400 mb-3" />
      <p className="text-warm-400 text-sm mb-4">{message}</p>
      {onRetry && (
        <button
          onClick={onRetry}
          className="btn-outline text-sm px-3 py-1"
        >
          <RefreshCw size={14} className="mr-1" />
          Retry
        </button>
      )}
    </div>
  )
}
