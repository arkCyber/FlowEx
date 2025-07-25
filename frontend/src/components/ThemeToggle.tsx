/**
 * FlowEx Theme Toggle Component
 * 
 * A beautiful theme toggle button with smooth animations
 */

import React from 'react'
import { Sun, Moon } from 'lucide-react'
import { useTheme } from '../hooks/useTheme'
import { cn } from '../utils/cn'

interface ThemeToggleProps {
  className?: string
  size?: 'sm' | 'md' | 'lg'
  showLabel?: boolean
}

export const ThemeToggle: React.FC<ThemeToggleProps> = ({
  className,
  size = 'md',
  showLabel = false,
}) => {
  const { isDarkMode, toggle } = useTheme()

  const sizeClasses = {
    sm: 'w-8 h-8',
    md: 'w-10 h-10',
    lg: 'w-12 h-12',
  }

  const iconSizes = {
    sm: 16,
    md: 20,
    lg: 24,
  }

  return (
    <div className={cn('flex items-center gap-2', className)}>
      {showLabel && (
        <span className="text-sm font-medium text-warm-300">
          {isDarkMode ? 'Dark' : 'Light'}
        </span>
      )}
      
      <button
        onClick={toggle}
        className={cn(
          'relative rounded-full border-2 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 focus:ring-offset-background-warm',
          sizeClasses[size],
          isDarkMode
            ? 'bg-background-warm-tertiary border-warm-600 hover:border-warm-500'
            : 'bg-yellow-100 border-yellow-300 hover:border-yellow-400'
        )}
        aria-label={`Switch to ${isDarkMode ? 'light' : 'dark'} mode`}
        title={`Switch to ${isDarkMode ? 'light' : 'dark'} mode`}
      >
        <div className="relative w-full h-full flex items-center justify-center">
          {/* Sun Icon */}
          <Sun
            size={iconSizes[size]}
            className={cn(
              'absolute transition-all duration-300 transform',
              isDarkMode
                ? 'opacity-0 scale-0 rotate-90 text-warm-600'
                : 'opacity-100 scale-100 rotate-0 text-yellow-600'
            )}
          />
          
          {/* Moon Icon */}
          <Moon
            size={iconSizes[size]}
            className={cn(
              'absolute transition-all duration-300 transform',
              isDarkMode
                ? 'opacity-100 scale-100 rotate-0 text-warm-400'
                : 'opacity-0 scale-0 -rotate-90 text-gray-400'
            )}
          />
        </div>
        
        {/* Glow effect for dark mode */}
        {isDarkMode && (
          <div className="absolute inset-0 rounded-full bg-warm-600 opacity-20 blur-sm animate-pulse-slow" />
        )}
      </button>
    </div>
  )
}

/**
 * Compact Theme Toggle for headers/toolbars
 */
export const CompactThemeToggle: React.FC<{ className?: string }> = ({ className }) => {
  const { isDarkMode, toggle } = useTheme()

  return (
    <button
      onClick={toggle}
      className={cn(
        'p-2 rounded-lg transition-all duration-200 hover:bg-warm-800 focus:outline-none focus:ring-2 focus:ring-primary-500',
        className
      )}
      aria-label={`Switch to ${isDarkMode ? 'light' : 'dark'} mode`}
    >
      {isDarkMode ? (
        <Sun size={18} className="text-warm-400 hover:text-warm-300" />
      ) : (
        <Moon size={18} className="text-gray-600 hover:text-gray-800" />
      )}
    </button>
  )
}

/**
 * Theme Toggle with Text
 */
export const ThemeToggleWithText: React.FC<{ className?: string }> = ({ className }) => {
  const { isDarkMode, toggle } = useTheme()

  return (
    <button
      onClick={toggle}
      className={cn(
        'flex items-center gap-3 px-4 py-2 rounded-lg transition-all duration-200 hover:bg-warm-800 focus:outline-none focus:ring-2 focus:ring-primary-500',
        className
      )}
    >
      {isDarkMode ? (
        <>
          <Sun size={20} className="text-warm-400" />
          <span className="text-warm-300 font-medium">Light Mode</span>
        </>
      ) : (
        <>
          <Moon size={20} className="text-gray-600" />
          <span className="text-gray-700 font-medium">Dark Mode</span>
        </>
      )}
    </button>
  )
}

/**
 * Animated Theme Toggle Switch
 */
export const ThemeToggleSwitch: React.FC<{ className?: string }> = ({ className }) => {
  const { isDarkMode, toggle } = useTheme()

  return (
    <div className={cn('flex items-center gap-3', className)}>
      <Sun size={16} className={cn('transition-colors', isDarkMode ? 'text-warm-600' : 'text-yellow-500')} />
      
      <button
        onClick={toggle}
        className={cn(
          'relative w-12 h-6 rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 focus:ring-offset-background-warm',
          isDarkMode ? 'bg-primary-600' : 'bg-gray-300'
        )}
        aria-label={`Switch to ${isDarkMode ? 'light' : 'dark'} mode`}
      >
        <div
          className={cn(
            'absolute top-0.5 w-5 h-5 bg-white rounded-full shadow-md transition-transform duration-300',
            isDarkMode ? 'translate-x-6' : 'translate-x-0.5'
          )}
        />
      </button>
      
      <Moon size={16} className={cn('transition-colors', isDarkMode ? 'text-warm-400' : 'text-gray-400')} />
    </div>
  )
}
