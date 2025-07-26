/**
 * Market Statistics Component
 * 
 * Displays key market metrics in a professional layout
 * Shows 24h high/low, volume, and other trading statistics
 */

import React from 'react'
import { useTheme } from '../../hooks/useTheme'
import { ArrowUpIcon, ArrowDownIcon } from '@heroicons/react/24/outline'

interface MarketData {
  symbol: string
  price: string
  change: string
  change_percent: string
  high: string
  low: string
  volume: string
  timestamp: string
}

interface TradingPair {
  symbol: string
  base_asset: string
  quote_asset: string
  status: string
}

interface MarketStatsProps {
  marketData?: MarketData
  tradingPair?: TradingPair
  className?: string
}

export const MarketStats: React.FC<MarketStatsProps> = ({
  marketData,
  tradingPair,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  
  if (!marketData || !tradingPair) {
    return null
  }
  
  const changeValue = parseFloat(marketData.change.replace(',', '') || '0')
  const changePercent = parseFloat(marketData.change_percent || '0')
  const isPositive = changeValue >= 0
  
  const stats = [
    {
      label: '24h Change',
      value: `${isPositive ? '+' : ''}${marketData.change}`,
      subValue: `${isPositive ? '+' : ''}${changePercent.toFixed(2)}%`,
      isPrice: true,
      trend: isPositive ? 'up' : 'down'
    },
    {
      label: '24h High',
      value: marketData.high,
      isPrice: true
    },
    {
      label: '24h Low',
      value: marketData.low,
      isPrice: true
    },
    {
      label: '24h Volume',
      value: marketData.volume,
      subValue: tradingPair.base_asset,
      isVolume: true
    }
  ]
  
  return (
    <div className={`flex items-center space-x-8 ${className}`}>
      {stats.map((stat, index) => (
        <div key={index} className="flex flex-col">
          <div className={`text-xs font-medium mb-1 ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            {stat.label}
          </div>
          
          <div className="flex items-center space-x-1">
            {/* Main Value */}
            <span className={`font-mono text-sm font-medium ${
              stat.trend === 'up' 
                ? 'text-success-500'
                : stat.trend === 'down'
                  ? 'text-danger-500'
                  : isDarkMode 
                    ? 'text-dark-50' 
                    : 'text-gray-900'
            }`}>
              {stat.isPrice && '$'}{stat.value}
            </span>
            
            {/* Trend Icon */}
            {stat.trend && (
              <div className={`flex items-center ${
                stat.trend === 'up' ? 'text-success-500' : 'text-danger-500'
              }`}>
                {stat.trend === 'up' ? (
                  <ArrowUpIcon className="w-3 h-3" />
                ) : (
                  <ArrowDownIcon className="w-3 h-3" />
                )}
              </div>
            )}
            
            {/* Sub Value */}
            {stat.subValue && (
              <span className={`text-xs ${
                stat.trend === 'up' 
                  ? 'text-success-500'
                  : stat.trend === 'down'
                    ? 'text-danger-500'
                    : isDarkMode 
                      ? 'text-dark-400' 
                      : 'text-gray-500'
              }`}>
                {stat.subValue}
              </span>
            )}
          </div>
        </div>
      ))}
      
      {/* Additional Market Info */}
      <div className="flex flex-col">
        <div className={`text-xs font-medium mb-1 ${
          isDarkMode ? 'text-dark-400' : 'text-gray-500'
        }`}>
          Status
        </div>
        <div className="flex items-center space-x-2">
          <div className="w-2 h-2 bg-success-500 rounded-full animate-pulse"></div>
          <span className={`text-xs font-medium ${
            isDarkMode ? 'text-success-400' : 'text-success-600'
          }`}>
            {tradingPair.status}
          </span>
        </div>
      </div>
    </div>
  )
}
