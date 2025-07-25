/**
 * FlowEx Tailwind CSS Utility Functions
 * 
 * Utility functions for combining and merging Tailwind CSS classes
 */

import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

/**
 * Combines and merges Tailwind CSS classes
 * @param inputs - Class values to combine
 * @returns Merged class string
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Conditional class utility
 * @param condition - Boolean condition
 * @param trueClasses - Classes to apply when true
 * @param falseClasses - Classes to apply when false
 * @returns Conditional classes
 */
export function conditionalClass(
  condition: boolean,
  trueClasses: string,
  falseClasses: string = ''
) {
  return condition ? trueClasses : falseClasses
}

/**
 * Trading price color utility
 * @param value - Current value
 * @param previousValue - Previous value for comparison
 * @returns Price color classes
 */
export function priceColor(value: number, previousValue?: number) {
  if (previousValue === undefined) return 'text-trading-neutral'
  
  if (value > previousValue) return 'text-trading-buy'
  if (value < previousValue) return 'text-trading-sell'
  return 'text-trading-neutral'
}

/**
 * Format percentage change with color
 * @param change - Percentage change
 * @returns Object with formatted text and color classes
 */
export function formatPercentageChange(change: number) {
  const isPositive = change > 0
  const isNegative = change < 0
  
  return {
    text: `${isPositive ? '+' : ''}${change.toFixed(2)}%`,
    className: cn(
      'font-medium',
      isPositive && 'text-trading-buy',
      isNegative && 'text-trading-sell',
      !isPositive && !isNegative && 'text-trading-neutral'
    )
  }
}

/**
 * Button variant classes
 */
export const buttonVariants = {
  primary: 'btn-primary',
  secondary: 'btn-secondary',
  outline: 'btn-outline',
  ghost: 'hover:bg-warm-800 text-warm-300 hover:text-white px-4 py-2 rounded-lg font-medium transition-all duration-200',
  danger: 'bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors duration-200',
  success: 'bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg font-medium transition-colors duration-200',
}

/**
 * Input variant classes
 */
export const inputVariants = {
  default: 'input',
  error: 'input border-red-500 focus:ring-red-500',
  success: 'input border-green-500 focus:ring-green-500',
}

/**
 * Card variant classes
 */
export const cardVariants = {
  default: 'card',
  elevated: 'card shadow-warm-lg',
  bordered: 'card border-2',
  trading: 'card border-primary-800 shadow-trading',
}
