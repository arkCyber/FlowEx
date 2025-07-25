/**
 * FlowEx Auth Layout Component with Tailwind CSS
 */

import React from 'react'
import { ThemeToggle } from '../ThemeToggle'
import { cn } from '../../utils/cn'

interface AuthLayoutProps {
  children: React.ReactNode
  className?: string
}

export const AuthLayout: React.FC<AuthLayoutProps> = ({ children, className }) => {
  return (
    <div className={cn(
      'min-h-screen bg-gradient-to-br from-background-warm via-background-warm-secondary to-background-warm-tertiary',
      'dark:from-background-warm dark:via-background-warm-secondary dark:to-background-warm-tertiary',
      'light:from-blue-50 light:via-indigo-50 light:to-purple-50',
      'flex items-center justify-center p-4',
      className
    )}>
      {/* Theme Toggle */}
      <div className="absolute top-6 right-6">
        <ThemeToggle size="md" showLabel />
      </div>

      {/* Background Pattern */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-40 -right-40 w-80 h-80 bg-primary-500 rounded-full opacity-10 blur-3xl animate-pulse-slow" />
        <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-warm-500 rounded-full opacity-10 blur-3xl animate-pulse-slow" />
      </div>

      {/* Content */}
      <div className="relative z-10 w-full max-w-md">
        {children}
      </div>
    </div>
  )
}
