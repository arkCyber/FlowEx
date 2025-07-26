/**
 * Professional Order Book Component
 * 
 * Real-time order book display with depth visualization
 * Supports both compact and depth chart modes
 */

import React, { useMemo, useState } from 'react'
import { useTheme } from '../../hooks/useTheme'
import { ArrowUpIcon, ArrowDownIcon } from '@heroicons/react/24/outline'

interface OrderBookEntry {
  price: number
  quantity: number
  total: number
}

interface OrderBookProps {
  symbol: string
  mode?: 'compact' | 'depth'
  className?: string
}

// Generate mock order book data
const generateMockOrderBook = (symbol: string) => {
  const basePrice = symbol === 'BTCUSDT' ? 45000 : symbol === 'ETHUSDT' ? 2800 : 0.45
  const spread = basePrice * 0.001 // 0.1% spread
  
  const asks: OrderBookEntry[] = []
  const bids: OrderBookEntry[] = []
  
  // Generate asks (sell orders)
  for (let i = 0; i < 20; i++) {
    const price = basePrice + spread + (i * basePrice * 0.0001)
    const quantity = Math.random() * 10 + 0.1
    const total = i === 0 ? quantity : asks[i - 1].total + quantity
    
    asks.push({
      price: parseFloat(price.toFixed(2)),
      quantity: parseFloat(quantity.toFixed(4)),
      total: parseFloat(total.toFixed(4))
    })
  }
  
  // Generate bids (buy orders)
  for (let i = 0; i < 20; i++) {
    const price = basePrice - spread - (i * basePrice * 0.0001)
    const quantity = Math.random() * 10 + 0.1
    const total = i === 0 ? quantity : bids[i - 1].total + quantity
    
    bids.push({
      price: parseFloat(price.toFixed(2)),
      quantity: parseFloat(quantity.toFixed(4)),
      total: parseFloat(total.toFixed(4))
    })
  }
  
  return { asks: asks.reverse(), bids }
}

// Order Book Row Component
const OrderBookRow: React.FC<{
  entry: OrderBookEntry
  type: 'ask' | 'bid'
  maxTotal: number
  onClick?: (price: number) => void
}> = ({ entry, type, maxTotal, onClick }) => {
  const { isDarkMode } = useTheme()
  const percentage = (entry.total / maxTotal) * 100
  
  return (
    <div
      className={`relative flex items-center justify-between px-3 py-1 text-xs cursor-pointer transition-colors ${
        isDarkMode ? 'hover:bg-dark-800/50' : 'hover:bg-gray-50'
      }`}
      onClick={() => onClick?.(entry.price)}
    >
      {/* Background bar */}
      <div
        className={`absolute inset-y-0 right-0 ${
          type === 'ask' 
            ? 'bg-danger-500/10' 
            : 'bg-success-500/10'
        }`}
        style={{ width: `${percentage}%` }}
      />
      
      {/* Content */}
      <div className="relative z-10 flex items-center justify-between w-full">
        <span className={`font-mono ${
          type === 'ask' ? 'text-danger-500' : 'text-success-500'
        }`}>
          {entry.price.toFixed(2)}
        </span>
        <span className={`font-mono ${
          isDarkMode ? 'text-dark-300' : 'text-gray-600'
        }`}>
          {entry.quantity.toFixed(4)}
        </span>
        <span className={`font-mono text-xs ${
          isDarkMode ? 'text-dark-400' : 'text-gray-500'
        }`}>
          {entry.total.toFixed(4)}
        </span>
      </div>
    </div>
  )
}

// Spread Display Component
const SpreadDisplay: React.FC<{
  bestAsk: number
  bestBid: number
}> = ({ bestAsk, bestBid }) => {
  const { isDarkMode } = useTheme()
  const spread = bestAsk - bestBid
  const spreadPercent = (spread / bestBid) * 100
  
  return (
    <div className={`flex items-center justify-center py-3 border-y ${
      isDarkMode ? 'border-dark-800 bg-dark-800/30' : 'border-gray-200 bg-gray-50'
    }`}>
      <div className="text-center">
        <div className={`text-xs font-medium ${
          isDarkMode ? 'text-dark-300' : 'text-gray-600'
        }`}>
          Spread
        </div>
        <div className="flex items-center space-x-2 text-xs">
          <span className="font-mono">{spread.toFixed(2)}</span>
          <span className={`${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            ({spreadPercent.toFixed(3)}%)
          </span>
        </div>
      </div>
    </div>
  )
}

export const OrderBook: React.FC<OrderBookProps> = ({
  symbol,
  mode = 'compact',
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [precision, setPrecision] = useState(2)
  const [showDepth, setShowDepth] = useState(10)
  
  const orderBook = useMemo(() => generateMockOrderBook(symbol), [symbol])
  
  const displayAsks = orderBook.asks.slice(0, showDepth)
  const displayBids = orderBook.bids.slice(0, showDepth)
  
  const maxTotal = Math.max(
    ...displayAsks.map(a => a.total),
    ...displayBids.map(b => b.total)
  )
  
  const bestAsk = displayAsks[displayAsks.length - 1]?.price || 0
  const bestBid = displayBids[0]?.price || 0
  
  const handlePriceClick = (price: number) => {
    // This would typically update the order form with the selected price
    console.log('Selected price:', price)
  }
  
  if (mode === 'depth') {
    // Depth chart mode - simplified for now
    return (
      <div className={`${className} flex flex-col`}>
        <div className={`p-4 border-b ${
          isDarkMode ? 'border-dark-800' : 'border-gray-200'
        }`}>
          <h3 className="font-semibold mb-2">Order Book Depth</h3>
          <div className={`text-center py-8 ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            Depth chart visualization coming soon...
          </div>
        </div>
      </div>
    )
  }
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Header */}
      <div className={`flex items-center justify-between p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <h3 className="font-semibold">Order Book</h3>
        
        <div className="flex items-center space-x-2">
          <select
            value={showDepth}
            onChange={(e) => setShowDepth(Number(e.target.value))}
            className={`text-xs px-2 py-1 rounded border ${
              isDarkMode
                ? 'bg-dark-800 border-dark-700 text-dark-200'
                : 'bg-white border-gray-200 text-gray-700'
            } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
          >
            <option value={5}>5</option>
            <option value={10}>10</option>
            <option value={20}>20</option>
          </select>
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
        <span>Total</span>
      </div>
      
      {/* Asks (Sell Orders) */}
      <div className="flex-1 overflow-y-auto">
        {displayAsks.map((ask, index) => (
          <OrderBookRow
            key={`ask-${index}`}
            entry={ask}
            type="ask"
            maxTotal={maxTotal}
            onClick={handlePriceClick}
          />
        ))}
      </div>
      
      {/* Spread */}
      <SpreadDisplay bestAsk={bestAsk} bestBid={bestBid} />
      
      {/* Bids (Buy Orders) */}
      <div className="flex-1 overflow-y-auto">
        {displayBids.map((bid, index) => (
          <OrderBookRow
            key={`bid-${index}`}
            entry={bid}
            type="bid"
            maxTotal={maxTotal}
            onClick={handlePriceClick}
          />
        ))}
      </div>
      
      {/* Market Summary */}
      <div className={`p-3 border-t ${
        isDarkMode ? 'border-dark-800 bg-dark-800/30' : 'border-gray-200 bg-gray-50'
      }`}>
        <div className="flex items-center justify-between text-xs">
          <div className="flex items-center space-x-1">
            <ArrowUpIcon className="w-3 h-3 text-success-500" />
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Best Ask:
            </span>
            <span className="font-mono text-danger-500">
              {bestAsk.toFixed(2)}
            </span>
          </div>
          
          <div className="flex items-center space-x-1">
            <ArrowDownIcon className="w-3 h-3 text-danger-500" />
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Best Bid:
            </span>
            <span className="font-mono text-success-500">
              {bestBid.toFixed(2)}
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}
