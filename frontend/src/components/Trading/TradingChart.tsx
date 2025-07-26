/**
 * Professional Trading Chart Component
 * 
 * Advanced candlestick chart with technical indicators
 * Inspired by TradingView and professional trading platforms
 */

import React, { useMemo, useState } from 'react'
import {
  ComposedChart,
  CandlestickChart,
  XAxis,
  YAxis,
  CartesianGrid,
  ResponsiveContainer,
  Line,
  Bar,
  ReferenceLine,
  Tooltip,
  Legend
} from 'recharts'
import { useTheme } from '../../hooks/useTheme'

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

interface TradingChartProps {
  symbol: string
  interval: string
  marketData?: MarketData
  className?: string
}

// Generate mock candlestick data
const generateMockData = (symbol: string, interval: string) => {
  const data = []
  const basePrice = symbol === 'BTCUSDT' ? 45000 : symbol === 'ETHUSDT' ? 2800 : 0.45
  let currentPrice = basePrice
  
  for (let i = 0; i < 100; i++) {
    const change = (Math.random() - 0.5) * basePrice * 0.02 // 2% max change
    const open = currentPrice
    const close = currentPrice + change
    const high = Math.max(open, close) + Math.random() * basePrice * 0.01
    const low = Math.min(open, close) - Math.random() * basePrice * 0.01
    const volume = Math.random() * 1000 + 100
    
    data.push({
      time: new Date(Date.now() - (100 - i) * 60000).toISOString(),
      open: parseFloat(open.toFixed(2)),
      high: parseFloat(high.toFixed(2)),
      low: parseFloat(low.toFixed(2)),
      close: parseFloat(close.toFixed(2)),
      volume: parseFloat(volume.toFixed(2)),
      ma5: 0,
      ma10: 0,
      ma20: 0
    })
    
    currentPrice = close
  }
  
  // Calculate moving averages
  for (let i = 0; i < data.length; i++) {
    if (i >= 4) {
      data[i].ma5 = data.slice(i - 4, i + 1).reduce((sum, item) => sum + item.close, 0) / 5
    }
    if (i >= 9) {
      data[i].ma10 = data.slice(i - 9, i + 1).reduce((sum, item) => sum + item.close, 0) / 10
    }
    if (i >= 19) {
      data[i].ma20 = data.slice(i - 19, i + 1).reduce((sum, item) => sum + item.close, 0) / 20
    }
  }
  
  return data
}

// Custom Candlestick Component
const Candlestick = ({ payload, x, y, width, height }: any) => {
  if (!payload) return null
  
  const { open, high, low, close } = payload
  const isPositive = close >= open
  const color = isPositive ? '#22c55e' : '#ef4444'
  const bodyHeight = Math.abs(close - open)
  const bodyY = Math.min(open, close)
  
  return (
    <g>
      {/* Wick */}
      <line
        x1={x + width / 2}
        y1={high}
        x2={x + width / 2}
        y2={low}
        stroke={color}
        strokeWidth={1}
      />
      {/* Body */}
      <rect
        x={x + width * 0.2}
        y={bodyY}
        width={width * 0.6}
        height={bodyHeight || 1}
        fill={isPositive ? color : color}
        stroke={color}
        strokeWidth={1}
      />
    </g>
  )
}

// Custom Tooltip
const CustomTooltip = ({ active, payload, label }: any) => {
  const { isDarkMode } = useTheme()
  
  if (!active || !payload || !payload.length) return null
  
  const data = payload[0].payload
  
  return (
    <div className={`p-3 rounded-lg shadow-lg border ${
      isDarkMode 
        ? 'bg-dark-800 border-dark-700 text-dark-50' 
        : 'bg-white border-gray-200 text-gray-900'
    }`}>
      <div className="text-xs font-medium mb-2">
        {new Date(label).toLocaleString()}
      </div>
      <div className="space-y-1 text-xs">
        <div className="flex justify-between space-x-4">
          <span>Open:</span>
          <span className="font-mono">{data.open?.toFixed(2)}</span>
        </div>
        <div className="flex justify-between space-x-4">
          <span>High:</span>
          <span className="font-mono text-success-500">{data.high?.toFixed(2)}</span>
        </div>
        <div className="flex justify-between space-x-4">
          <span>Low:</span>
          <span className="font-mono text-danger-500">{data.low?.toFixed(2)}</span>
        </div>
        <div className="flex justify-between space-x-4">
          <span>Close:</span>
          <span className={`font-mono ${
            data.close >= data.open ? 'text-success-500' : 'text-danger-500'
          }`}>
            {data.close?.toFixed(2)}
          </span>
        </div>
        <div className="flex justify-between space-x-4">
          <span>Volume:</span>
          <span className="font-mono">{data.volume?.toFixed(2)}</span>
        </div>
      </div>
    </div>
  )
}

export const TradingChart: React.FC<TradingChartProps> = ({
  symbol,
  interval,
  marketData,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [showMA, setShowMA] = useState({ ma5: true, ma10: true, ma20: false })
  const [showVolume, setShowVolume] = useState(true)
  
  const chartData = useMemo(() => 
    generateMockData(symbol, interval), 
    [symbol, interval]
  )
  
  const currentPrice = marketData ? parseFloat(marketData.price.replace(',', '')) : 0
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Chart Controls */}
      <div className={`flex items-center justify-between p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <div className="flex items-center space-x-4">
          <h3 className="font-semibold">{symbol} Chart</h3>
          <div className="flex items-center space-x-2 text-xs">
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Interval: {interval}
            </span>
          </div>
        </div>
        
        {/* Indicators Toggle */}
        <div className="flex items-center space-x-2">
          <div className="flex items-center space-x-1">
            <input
              type="checkbox"
              id="ma5"
              checked={showMA.ma5}
              onChange={(e) => setShowMA(prev => ({ ...prev, ma5: e.target.checked }))}
              className="w-3 h-3"
            />
            <label htmlFor="ma5" className="text-xs text-chart-ma5">MA5</label>
          </div>
          
          <div className="flex items-center space-x-1">
            <input
              type="checkbox"
              id="ma10"
              checked={showMA.ma10}
              onChange={(e) => setShowMA(prev => ({ ...prev, ma10: e.target.checked }))}
              className="w-3 h-3"
            />
            <label htmlFor="ma10" className="text-xs text-chart-ma10">MA10</label>
          </div>
          
          <div className="flex items-center space-x-1">
            <input
              type="checkbox"
              id="ma20"
              checked={showMA.ma20}
              onChange={(e) => setShowMA(prev => ({ ...prev, ma20: e.target.checked }))}
              className="w-3 h-3"
            />
            <label htmlFor="ma20" className="text-xs text-chart-ma20">MA20</label>
          </div>
          
          <div className="flex items-center space-x-1">
            <input
              type="checkbox"
              id="volume"
              checked={showVolume}
              onChange={(e) => setShowVolume(e.target.checked)}
              className="w-3 h-3"
            />
            <label htmlFor="volume" className="text-xs">Volume</label>
          </div>
        </div>
      </div>
      
      {/* Chart */}
      <div className="flex-1 p-4">
        <ResponsiveContainer width="100%" height="100%">
          <ComposedChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
            <CartesianGrid 
              strokeDasharray="3 3" 
              stroke={isDarkMode ? '#2a2a2a' : '#e5e7eb'} 
            />
            <XAxis 
              dataKey="time"
              tickFormatter={(value) => new Date(value).toLocaleTimeString([], { 
                hour: '2-digit', 
                minute: '2-digit' 
              })}
              stroke={isDarkMode ? '#6b7280' : '#374151'}
              fontSize={12}
            />
            <YAxis 
              domain={['dataMin - 50', 'dataMax + 50']}
              stroke={isDarkMode ? '#6b7280' : '#374151'}
              fontSize={12}
              tickFormatter={(value) => `$${value.toFixed(0)}`}
            />
            
            {/* Current Price Line */}
            {currentPrice > 0 && (
              <ReferenceLine 
                y={currentPrice} 
                stroke="#f59e0b" 
                strokeDasharray="5 5"
                strokeWidth={2}
              />
            )}
            
            {/* Moving Averages */}
            {showMA.ma5 && (
              <Line 
                type="monotone" 
                dataKey="ma5" 
                stroke="#f59e0b" 
                strokeWidth={1}
                dot={false}
                connectNulls={false}
              />
            )}
            {showMA.ma10 && (
              <Line 
                type="monotone" 
                dataKey="ma10" 
                stroke="#8b5cf6" 
                strokeWidth={1}
                dot={false}
                connectNulls={false}
              />
            )}
            {showMA.ma20 && (
              <Line 
                type="monotone" 
                dataKey="ma20" 
                stroke="#06b6d4" 
                strokeWidth={1}
                dot={false}
                connectNulls={false}
              />
            )}
            
            {/* Volume Bars */}
            {showVolume && (
              <Bar 
                dataKey="volume" 
                fill="#4a5568" 
                opacity={0.3}
                yAxisId="volume"
              />
            )}
            
            <Tooltip content={<CustomTooltip />} />
          </ComposedChart>
        </ResponsiveContainer>
      </div>
    </div>
  )
}
