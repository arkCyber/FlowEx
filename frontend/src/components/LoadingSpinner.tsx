/**
 * FlowEx Loading Spinner Component with Tailwind CSS
 */

import React from 'react'
import { Loader2 } from 'lucide-react'
import { cn } from '../utils/cn'

interface LoadingSpinnerProps {
  message?: string
  size?: 'sm' | 'md' | 'lg' | 'xl'
  fullScreen?: boolean
  className?: string
}

export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  message = 'Loading...',
  size = 'md',
  fullScreen = false,
  className,
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-6 h-6',
    lg: 'w-8 h-8',
    xl: 'w-12 h-12',
  }

  const content = (
    <div className="flex flex-col items-center justify-center gap-3">
      <Loader2
        className={cn(
          'animate-spin text-primary-500',
          sizeClasses[size]
        )}
      />
      {message && (
        <p className="text-sm text-warm-400 font-medium animate-pulse">
          {message}
        </p>
      )}
    </div>
  )

  if (fullScreen) {
    return (
      <div className={cn(
        'fixed inset-0 bg-background-warm bg-opacity-80 backdrop-blur-sm flex items-center justify-center z-50',
        className
      )}>
        {content}
      </div>
    )
  }

  return (
    <div className={cn(
      'flex items-center justify-center min-h-48',
      className
    )}>
      {content}
    </div>
  )
}

/**
 * Inline Loading Spinner
 */
export const InlineSpinner: React.FC<{ size?: 'sm' | 'md'; className?: string }> = ({
  size = 'sm',
  className
}) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
  }

  return (
    <Loader2
      className={cn(
        'animate-spin text-current',
        sizeClasses[size],
        className
      )}
    />
  )
}

/**
 * Button Loading Spinner
 */
export const ButtonSpinner: React.FC<{ className?: string }> = ({ className }) => {
  return (
    <Loader2
      className={cn(
        'animate-spin w-4 h-4 text-current',
        className
      )}
    />
  )
}
