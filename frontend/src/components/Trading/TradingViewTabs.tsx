/**
 * Trading View Tabs Component
 * 
 * Tab navigation for different trading views (Chart, Depth, Trades)
 * Includes time interval selector for charts
 */

import React from 'react'
import { useTheme } from '../../hooks/useTheme'
import { 
  ChartBarIcon, 
  ChartPieIcon, 
  ArrowsRightLeftIcon,
  ClockIcon
} from '@heroicons/react/24/outline'

interface TradingViewTabsProps {
  activeTab: 'chart' | 'depth' | 'trades'
  onTabChange: (tab: 'chart' | 'depth' | 'trades') => void
  interval: string
  onIntervalChange: (interval: string) => void
  className?: string
}

export const TradingViewTabs: React.FC<TradingViewTabsProps> = ({
  activeTab,
  onTabChange,
  interval,
  onIntervalChange,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  
  const tabs = [
    {
      key: 'chart' as const,
      label: 'Chart',
      icon: ChartBarIcon,
      description: 'Price chart with technical indicators'
    },
    {
      key: 'depth' as const,
      label: 'Depth',
      icon: ChartPieIcon,
      description: 'Order book depth visualization'
    },
    {
      key: 'trades' as const,
      label: 'Trades',
      icon: ArrowsRightLeftIcon,
      description: 'Recent trades history'
    }
  ]
  
  const intervals = [
    { value: '1m', label: '1m' },
    { value: '5m', label: '5m' },
    { value: '15m', label: '15m' },
    { value: '30m', label: '30m' },
    { value: '1h', label: '1h' },
    { value: '4h', label: '4h' },
    { value: '1d', label: '1D' },
    { value: '1w', label: '1W' }
  ]
  
  return (
    <div className={`flex items-center justify-between ${className}`}>
      {/* View Tabs */}
      <div className={`flex items-center rounded-lg p-1 ${
        isDarkMode ? 'bg-dark-800' : 'bg-gray-100'
      }`}>
        {tabs.map(tab => {
          const Icon = tab.icon
          const isActive = activeTab === tab.key
          
          return (
            <button
              key={tab.key}
              onClick={() => onTabChange(tab.key)}
              className={`flex items-center space-x-2 px-4 py-2 rounded-md transition-all duration-200 ${
                isActive
                  ? isDarkMode
                    ? 'bg-dark-700 text-primary-400 shadow-sm'
                    : 'bg-white text-primary-600 shadow-sm'
                  : isDarkMode
                    ? 'text-dark-300 hover:text-dark-100 hover:bg-dark-700/50'
                    : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
              }`}
              title={tab.description}
            >
              <Icon className="w-4 h-4" />
              <span className="text-sm font-medium">{tab.label}</span>
            </button>
          )
        })}
      </div>
      
      {/* Time Interval Selector (only show for chart) */}
      {activeTab === 'chart' && (
        <div className="flex items-center space-x-2">
          <div className={`flex items-center space-x-1 text-xs ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            <ClockIcon className="w-3 h-3" />
            <span>Interval:</span>
          </div>
          
          <div className={`flex items-center rounded-lg p-1 ${
            isDarkMode ? 'bg-dark-800' : 'bg-gray-100'
          }`}>
            {intervals.map(int => (
              <button
                key={int.value}
                onClick={() => onIntervalChange(int.value)}
                className={`px-3 py-1 text-xs font-medium rounded-md transition-all duration-200 ${
                  interval === int.value
                    ? isDarkMode
                      ? 'bg-dark-700 text-primary-400'
                      : 'bg-white text-primary-600 shadow-sm'
                    : isDarkMode
                      ? 'text-dark-300 hover:text-dark-100 hover:bg-dark-700/50'
                      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
                }`}
              >
                {int.label}
              </button>
            ))}
          </div>
        </div>
      )}
      
      {/* Additional Controls for other tabs */}
      {activeTab === 'depth' && (
        <div className="flex items-center space-x-2">
          <div className={`text-xs ${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Precision:
          </div>
          <select
            className={`text-xs px-2 py-1 rounded border ${
              isDarkMode
                ? 'bg-dark-800 border-dark-700 text-dark-200'
                : 'bg-white border-gray-200 text-gray-700'
            } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
          >
            <option value="0.01">0.01</option>
            <option value="0.1">0.1</option>
            <option value="1">1</option>
            <option value="10">10</option>
          </select>
        </div>
      )}
      
      {activeTab === 'trades' && (
        <div className="flex items-center space-x-2">
          <div className={`text-xs ${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Show:
          </div>
          <select
            className={`text-xs px-2 py-1 rounded border ${
              isDarkMode
                ? 'bg-dark-800 border-dark-700 text-dark-200'
                : 'bg-white border-gray-200 text-gray-700'
            } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
          >
            <option value="all">All Trades</option>
            <option value="buy">Buy Orders</option>
            <option value="sell">Sell Orders</option>
          </select>
        </div>
      )}
    </div>
  )
}
