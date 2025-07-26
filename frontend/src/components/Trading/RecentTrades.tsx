/**
 * Recent Trades Component
 * 
 * Real-time display of recent trades with price and volume information
 * Shows market activity and trade flow
 */

import React, { useMemo, useState } from 'react'
import { useTheme } from '../../hooks/useTheme'
import { ArrowUpIcon, ArrowDownIcon, ClockIcon } from '@heroicons/react/24/outline'

interface Trade {
  id: string
  price: number
  quantity: number
  side: 'buy' | 'sell'
  timestamp: string
  total: number
}

interface RecentTradesProps {
  symbol: string
  className?: string
}

// Generate mock trade data
const generateMockTrades = (symbol: string): Trade[] => {
  const basePrice = symbol === 'BTCUSDT' ? 45000 : symbol === 'ETHUSDT' ? 2800 : 0.45
  const trades: Trade[] = []
  
  for (let i = 0; i < 50; i++) {
    const priceVariation = (Math.random() - 0.5) * basePrice * 0.002 // 0.2% variation
    const price = basePrice + priceVariation
    const quantity = Math.random() * 2 + 0.001
    const side = Math.random() > 0.5 ? 'buy' : 'sell'
    const timestamp = new Date(Date.now() - i * 1000 * Math.random() * 60).toISOString()
    
    trades.push({
      id: `trade-${i}`,
      price: parseFloat(price.toFixed(2)),
      quantity: parseFloat(quantity.toFixed(6)),
      side,
      timestamp,
      total: parseFloat((price * quantity).toFixed(2))
    })
  }
  
  return trades.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
}

// Trade Row Component
const TradeRow: React.FC<{
  trade: Trade
  isLatest?: boolean
}> = ({ trade, isLatest }) => {
  const { isDarkMode } = useTheme()
  
  return (
    <div className={`flex items-center justify-between px-3 py-1.5 text-xs transition-colors ${
      isLatest 
        ? trade.side === 'buy'
          ? 'bg-success-500/10 animate-price-up'
          : 'bg-danger-500/10 animate-price-down'
        : isDarkMode 
          ? 'hover:bg-dark-800/50' 
          : 'hover:bg-gray-50'
    }`}>
      {/* Price */}
      <div className="flex items-center space-x-1">
        <span className={`font-mono font-medium ${
          trade.side === 'buy' ? 'text-success-500' : 'text-danger-500'
        }`}>
          {trade.price.toFixed(2)}
        </span>
        {trade.side === 'buy' ? (
          <ArrowUpIcon className="w-3 h-3 text-success-500" />
        ) : (
          <ArrowDownIcon className="w-3 h-3 text-danger-500" />
        )}
      </div>
      
      {/* Quantity */}
      <span className={`font-mono ${
        isDarkMode ? 'text-dark-300' : 'text-gray-600'
      }`}>
        {trade.quantity.toFixed(6)}
      </span>
      
      {/* Time */}
      <span className={`font-mono text-xs ${
        isDarkMode ? 'text-dark-400' : 'text-gray-500'
      }`}>
        {new Date(trade.timestamp).toLocaleTimeString([], { 
          hour: '2-digit', 
          minute: '2-digit',
          second: '2-digit'
        })}
      </span>
    </div>
  )
}

// Trade Summary Component
const TradeSummary: React.FC<{
  trades: Trade[]
  symbol: string
}> = ({ trades, symbol }) => {
  const { isDarkMode } = useTheme()
  
  const summary = useMemo(() => {
    const recentTrades = trades.slice(0, 20) // Last 20 trades
    const buyTrades = recentTrades.filter(t => t.side === 'buy')
    const sellTrades = recentTrades.filter(t => t.side === 'sell')
    
    const buyVolume = buyTrades.reduce((sum, t) => sum + t.total, 0)
    const sellVolume = sellTrades.reduce((sum, t) => sum + t.total, 0)
    const totalVolume = buyVolume + sellVolume
    
    const buyPercentage = totalVolume > 0 ? (buyVolume / totalVolume) * 100 : 50
    const sellPercentage = totalVolume > 0 ? (sellVolume / totalVolume) * 100 : 50
    
    const avgPrice = recentTrades.length > 0 
      ? recentTrades.reduce((sum, t) => sum + t.price, 0) / recentTrades.length 
      : 0
    
    return {
      buyVolume,
      sellVolume,
      totalVolume,
      buyPercentage,
      sellPercentage,
      avgPrice,
      tradeCount: recentTrades.length
    }
  }, [trades])
  
  return (
    <div className={`p-3 border-t space-y-3 ${
      isDarkMode ? 'border-dark-800 bg-dark-800/30' : 'border-gray-200 bg-gray-50'
    }`}>
      {/* Volume Distribution */}
      <div>
        <div className={`text-xs font-medium mb-2 ${
          isDarkMode ? 'text-dark-300' : 'text-gray-600'
        }`}>
          Volume Distribution (Last 20 trades)
        </div>
        <div className="flex rounded-lg overflow-hidden h-2">
          <div 
            className="bg-success-500" 
            style={{ width: `${summary.buyPercentage}%` }}
          />
          <div 
            className="bg-danger-500" 
            style={{ width: `${summary.sellPercentage}%` }}
          />
        </div>
        <div className="flex justify-between mt-1 text-xs">
          <span className="text-success-500">
            Buy: {summary.buyPercentage.toFixed(1)}%
          </span>
          <span className="text-danger-500">
            Sell: {summary.sellPercentage.toFixed(1)}%
          </span>
        </div>
      </div>
      
      {/* Summary Stats */}
      <div className="grid grid-cols-2 gap-3 text-xs">
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Avg Price
          </div>
          <div className="font-mono font-medium">
            ${summary.avgPrice.toFixed(2)}
          </div>
        </div>
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Total Volume
          </div>
          <div className="font-mono font-medium">
            ${summary.totalVolume.toFixed(2)}
          </div>
        </div>
      </div>
    </div>
  )
}

export const RecentTrades: React.FC<RecentTradesProps> = ({
  symbol,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [filter, setFilter] = useState<'all' | 'buy' | 'sell'>('all')
  
  const trades = useMemo(() => generateMockTrades(symbol), [symbol])
  
  const filteredTrades = useMemo(() => {
    if (filter === 'all') return trades
    return trades.filter(trade => trade.side === filter)
  }, [trades, filter])
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Header */}
      <div className={`flex items-center justify-between p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <h3 className="font-semibold">Recent Trades</h3>
        
        {/* Filter Buttons */}
        <div className={`flex items-center rounded-lg p-1 ${
          isDarkMode ? 'bg-dark-800' : 'bg-gray-100'
        }`}>
          {[
            { key: 'all', label: 'All' },
            { key: 'buy', label: 'Buy' },
            { key: 'sell', label: 'Sell' }
          ].map(filterOption => (
            <button
              key={filterOption.key}
              onClick={() => setFilter(filterOption.key as any)}
              className={`px-3 py-1 text-xs font-medium rounded-md transition-colors ${
                filter === filterOption.key
                  ? isDarkMode
                    ? 'bg-dark-700 text-primary-400'
                    : 'bg-white text-primary-600 shadow-sm'
                  : isDarkMode
                    ? 'text-dark-300 hover:text-dark-100'
                    : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              {filterOption.label}
            </button>
          ))}
        </div>
      </div>
      
      {/* Column Headers */}
      <div className={`flex items-center justify-between px-3 py-2 text-xs font-medium border-b ${
        isDarkMode 
          ? 'border-dark-800 text-dark-400 bg-dark-800/30' 
          : 'border-gray-200 text-gray-500 bg-gray-50'
      }`}>
        <span>Price</span>
        <span>Amount</span>
        <span>Time</span>
      </div>
      
      {/* Trades List */}
      <div className="flex-1 overflow-y-auto">
        {filteredTrades.length === 0 ? (
          <div className={`p-8 text-center ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            <ClockIcon className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div className="text-sm">No trades found</div>
            <div className="text-xs mt-1">
              {filter !== 'all' ? `No ${filter} trades available` : 'Waiting for trades...'}
            </div>
          </div>
        ) : (
          filteredTrades.map((trade, index) => (
            <TradeRow
              key={trade.id}
              trade={trade}
              isLatest={index === 0}
            />
          ))
        )}
      </div>
      
      {/* Trade Summary */}
      <TradeSummary trades={trades} symbol={symbol} />
    </div>
  )
}
