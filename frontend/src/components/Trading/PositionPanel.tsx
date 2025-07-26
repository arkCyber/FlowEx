/**
 * Position Panel Component
 * 
 * Displays user's current positions and portfolio summary
 * Shows P&L, asset allocation, and position management
 */

import React, { useState, useMemo } from 'react'
import { useTheme } from '../../hooks/useTheme'
import { 
  CurrencyDollarIcon, 
  TrendingUpIcon,
  TrendingDownIcon,
  EyeIcon,
  EyeSlashIcon,
  ChartPieIcon
} from '@heroicons/react/24/outline'

interface Position {
  asset: string
  balance: number
  locked: number
  available: number
  usdValue: number
  price: number
  change24h: number
  percentage: number
}

interface PositionPanelProps {
  className?: string
}

// Generate mock position data
const generateMockPositions = (): Position[] => {
  const assets = [
    { symbol: 'BTC', price: 45234.56, balance: 0.12345678 },
    { symbol: 'ETH', price: 2834.12, balance: 2.5678 },
    { symbol: 'USDT', price: 1.00, balance: 10000.50 },
    { symbol: 'ADA', price: 0.4567, balance: 1500.25 },
    { symbol: 'DOT', price: 12.34, balance: 100.75 },
    { symbol: 'LINK', price: 15.67, balance: 50.25 },
    { symbol: 'UNI', price: 8.90, balance: 25.50 }
  ]
  
  const totalValue = assets.reduce((sum, asset) => sum + (asset.price * asset.balance), 0)
  
  return assets.map(asset => {
    const usdValue = asset.price * asset.balance
    const locked = asset.balance * (Math.random() * 0.1) // 0-10% locked
    const available = asset.balance - locked
    const change24h = (Math.random() - 0.5) * 10 // -5% to +5%
    const percentage = (usdValue / totalValue) * 100
    
    return {
      asset: asset.symbol,
      balance: asset.balance,
      locked: parseFloat(locked.toFixed(6)),
      available: parseFloat(available.toFixed(6)),
      usdValue: parseFloat(usdValue.toFixed(2)),
      price: asset.price,
      change24h: parseFloat(change24h.toFixed(2)),
      percentage: parseFloat(percentage.toFixed(2))
    }
  }).filter(position => position.usdValue > 1) // Filter out dust
    .sort((a, b) => b.usdValue - a.usdValue)
}

// Position Row Component
const PositionRow: React.FC<{
  position: Position
  showBalance: boolean
}> = ({ position, showBalance }) => {
  const { isDarkMode } = useTheme()
  const isPositive = position.change24h >= 0
  
  return (
    <div className={`p-3 border-b transition-colors ${
      isDarkMode 
        ? 'border-dark-800 hover:bg-dark-800/30' 
        : 'border-gray-200 hover:bg-gray-50'
    }`}>
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center space-x-3">
          <div className={`w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold ${
            isDarkMode ? 'bg-dark-700 text-dark-200' : 'bg-gray-100 text-gray-700'
          }`}>
            {position.asset.slice(0, 2)}
          </div>
          <div>
            <div className="font-medium text-sm">{position.asset}</div>
            <div className={`text-xs ${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
              ${position.price.toFixed(position.price < 1 ? 4 : 2)}
            </div>
          </div>
        </div>
        
        <div className="text-right">
          <div className="font-mono text-sm">
            {showBalance ? `$${position.usdValue.toFixed(2)}` : '****'}
          </div>
          <div className={`text-xs flex items-center space-x-1 ${
            isPositive ? 'text-success-500' : 'text-danger-500'
          }`}>
            {isPositive ? (
              <TrendingUpIcon className="w-3 h-3" />
            ) : (
              <TrendingDownIcon className="w-3 h-3" />
            )}
            <span>{isPositive ? '+' : ''}{position.change24h.toFixed(2)}%</span>
          </div>
        </div>
      </div>
      
      <div className="grid grid-cols-3 gap-3 text-xs">
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Balance
          </div>
          <div className="font-mono">
            {showBalance ? position.balance.toFixed(6) : '****'}
          </div>
        </div>
        
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Available
          </div>
          <div className="font-mono">
            {showBalance ? position.available.toFixed(6) : '****'}
          </div>
        </div>
        
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Allocation
          </div>
          <div className="flex items-center space-x-2">
            <div className="font-mono">{position.percentage.toFixed(1)}%</div>
            <div className="flex-1 bg-gray-200 rounded-full h-1">
              <div 
                className="bg-primary-500 h-1 rounded-full"
                style={{ width: `${Math.min(position.percentage, 100)}%` }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

// Portfolio Summary Component
const PortfolioSummary: React.FC<{
  positions: Position[]
  showBalance: boolean
}> = ({ positions, showBalance }) => {
  const { isDarkMode } = useTheme()
  
  const summary = useMemo(() => {
    const totalValue = positions.reduce((sum, pos) => sum + pos.usdValue, 0)
    const totalChange = positions.reduce((sum, pos) => {
      const dayChange = (pos.change24h / 100) * pos.usdValue
      return sum + dayChange
    }, 0)
    const totalChangePercent = totalValue > 0 ? (totalChange / totalValue) * 100 : 0
    
    const topAssets = positions.slice(0, 3)
    
    return {
      totalValue,
      totalChange,
      totalChangePercent,
      topAssets,
      assetCount: positions.length
    }
  }, [positions])
  
  const isPositive = summary.totalChangePercent >= 0
  
  return (
    <div className={`p-4 border-b ${
      isDarkMode ? 'border-dark-800 bg-dark-800/30' : 'border-gray-200 bg-gray-50'
    }`}>
      {/* Total Portfolio Value */}
      <div className="mb-4">
        <div className={`text-sm font-medium mb-1 ${
          isDarkMode ? 'text-dark-300' : 'text-gray-600'
        }`}>
          Total Portfolio Value
        </div>
        <div className="flex items-center space-x-3">
          <div className="text-2xl font-bold font-mono">
            {showBalance ? `$${summary.totalValue.toFixed(2)}` : '$****'}
          </div>
          <div className={`flex items-center space-x-1 text-sm ${
            isPositive ? 'text-success-500' : 'text-danger-500'
          }`}>
            {isPositive ? (
              <TrendingUpIcon className="w-4 h-4" />
            ) : (
              <TrendingDownIcon className="w-4 h-4" />
            )}
            <span>
              {isPositive ? '+' : ''}${Math.abs(summary.totalChange).toFixed(2)} 
              ({isPositive ? '+' : ''}{summary.totalChangePercent.toFixed(2)}%)
            </span>
          </div>
        </div>
      </div>
      
      {/* Top Assets */}
      <div>
        <div className={`text-sm font-medium mb-2 ${
          isDarkMode ? 'text-dark-300' : 'text-gray-600'
        }`}>
          Top Holdings ({summary.assetCount} assets)
        </div>
        <div className="flex items-center space-x-4">
          {summary.topAssets.map(asset => (
            <div key={asset.asset} className="flex items-center space-x-2">
              <div className={`w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold ${
                isDarkMode ? 'bg-dark-700 text-dark-200' : 'bg-gray-100 text-gray-700'
              }`}>
                {asset.asset.slice(0, 2)}
              </div>
              <div className="text-xs">
                <div className="font-medium">{asset.asset}</div>
                <div className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
                  {asset.percentage.toFixed(1)}%
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

export const PositionPanel: React.FC<PositionPanelProps> = ({
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [showBalance, setShowBalance] = useState(true)
  const [sortBy, setSortBy] = useState<'value' | 'change' | 'name'>('value')
  
  const positions = useMemo(() => generateMockPositions(), [])
  
  const sortedPositions = useMemo(() => {
    const sorted = [...positions]
    
    switch (sortBy) {
      case 'value':
        return sorted.sort((a, b) => b.usdValue - a.usdValue)
      case 'change':
        return sorted.sort((a, b) => b.change24h - a.change24h)
      case 'name':
        return sorted.sort((a, b) => a.asset.localeCompare(b.asset))
      default:
        return sorted
    }
  }, [positions, sortBy])
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Header */}
      <div className={`flex items-center justify-between p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <h3 className="font-semibold">Portfolio</h3>
        
        <div className="flex items-center space-x-2">
          {/* Sort Dropdown */}
          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as any)}
            className={`text-xs px-2 py-1 rounded border ${
              isDarkMode
                ? 'bg-dark-800 border-dark-700 text-dark-200'
                : 'bg-white border-gray-200 text-gray-700'
            } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
          >
            <option value="value">By Value</option>
            <option value="change">By Change</option>
            <option value="name">By Name</option>
          </select>
          
          {/* Balance Toggle */}
          <button
            onClick={() => setShowBalance(!showBalance)}
            className={`p-2 rounded-lg transition-colors ${
              isDarkMode
                ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
            }`}
            title={showBalance ? 'Hide Balance' : 'Show Balance'}
          >
            {showBalance ? (
              <EyeIcon className="w-4 h-4" />
            ) : (
              <EyeSlashIcon className="w-4 h-4" />
            )}
          </button>
        </div>
      </div>
      
      {/* Portfolio Summary */}
      <PortfolioSummary positions={positions} showBalance={showBalance} />
      
      {/* Positions List */}
      <div className="flex-1 overflow-y-auto">
        {sortedPositions.length === 0 ? (
          <div className={`p-8 text-center ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            <ChartPieIcon className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div className="text-sm">No positions found</div>
            <div className="text-xs mt-1">Start trading to build your portfolio</div>
          </div>
        ) : (
          sortedPositions.map(position => (
            <PositionRow
              key={position.asset}
              position={position}
              showBalance={showBalance}
            />
          ))
        )}
      </div>
    </div>
  )
}
