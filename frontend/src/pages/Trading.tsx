/**
 * FlowEx Professional Trading Interface
 *
 * Enterprise-grade trading interface inspired by Binance and OKX
 * Features: Real-time charts, order book, trading forms, market data
 * Design: Dark mode with warm colors, professional layout
 */

import React, { useState, useEffect, useMemo } from 'react'
import { useParams, useNavigate } from 'react-router-dom'

// Components
import { TradingChart } from '../components/Trading/TradingChart'
import { OrderBook } from '../components/Trading/OrderBook'
import { OrderForm } from '../components/Trading/OrderForm'
import { RecentTrades } from '../components/Trading/RecentTrades'
import { TradingPairSelector } from '../components/Trading/TradingPairSelector'
import { MarketStats } from '../components/Trading/MarketStats'
import { TradingViewTabs } from '../components/Trading/TradingViewTabs'
import { OrderHistory } from '../components/Trading/OrderHistory'
import { PositionPanel } from '../components/Trading/PositionPanel'
import { LoadingSpinner } from '../components/LoadingSpinner'

// Hooks
import { useWebSocket } from '../hooks/useWebSocket'
import { useMarketData } from '../hooks/useMarketData'
import { useTradingPairs } from '../hooks/useTradingPairs'
import { useTheme } from '../hooks/useTheme'

// Types
import type { TradingPair, MarketData, Order } from '../types'

// Icons
import {
  ChartBarIcon,
  CurrencyDollarIcon,
  ClockIcon,
  Cog6ToothIcon,
  ArrowsRightLeftIcon,
  EyeIcon,
  EyeSlashIcon
} from '@heroicons/react/24/outline'

const Trading: React.FC = () => {
  const { symbol } = useParams<{ symbol?: string }>()
  const navigate = useNavigate()
  const { isDarkMode } = useTheme()

  // State
  const [selectedPair, setSelectedPair] = useState<string>(symbol || 'BTCUSDT')
  const [orderType, setOrderType] = useState<'buy' | 'sell'>('buy')
  const [activeTab, setActiveTab] = useState<'chart' | 'depth' | 'trades'>('chart')
  const [showOrderHistory, setShowOrderHistory] = useState(true)
  const [showPositions, setShowPositions] = useState(true)
  const [chartInterval, setChartInterval] = useState('1h')
  const [isFullscreen, setIsFullscreen] = useState(false)

  // Mock data for development
  const mockMarketData = {
    symbol: selectedPair,
    price: '45,234.56',
    change: '1,234.56',
    change_percent: '2.81',
    high: '46,789.12',
    low: '43,567.89',
    volume: '12,345.67',
    timestamp: new Date().toISOString()
  }

  const mockTradingPairs = [
    { symbol: 'BTCUSDT', base_asset: 'BTC', quote_asset: 'USDT', status: 'TRADING' },
    { symbol: 'ETHUSDT', base_asset: 'ETH', quote_asset: 'USDT', status: 'TRADING' },
    { symbol: 'ADAUSDT', base_asset: 'ADA', quote_asset: 'USDT', status: 'TRADING' },
    { symbol: 'DOTUSDT', base_asset: 'DOT', quote_asset: 'USDT', status: 'TRADING' },
  ]

  // Memoized values
  const currentPair = useMemo(() =>
    mockTradingPairs?.find(pair => pair.symbol === selectedPair),
    [selectedPair]
  )

  const priceChange = useMemo(() => {
    const change = parseFloat(mockMarketData.change.replace(',', '') || '0')
    const changePercent = parseFloat(mockMarketData.change_percent || '0')

    return {
      value: change,
      percentage: changePercent,
      isPositive: change >= 0
    }
  }, [mockMarketData])

  // Handle pair selection
  const handlePairChange = (newPair: string) => {
    setSelectedPair(newPair)
    navigate(`/trading/${newPair}`, { replace: true })
  }

  // Handle fullscreen toggle
  const toggleFullscreen = () => {
    setIsFullscreen(!isFullscreen)
  }

  return (
    <div className={`min-h-screen transition-colors duration-200 ${
      isDarkMode
        ? 'bg-dark-950 text-dark-50'
        : 'bg-gray-50 text-gray-900'
    }`}>
      {/* Trading Header */}
      <div className={`border-b sticky top-0 z-50 backdrop-blur-md ${
        isDarkMode
          ? 'border-dark-800 bg-dark-950/95'
          : 'border-gray-200 bg-white/95'
      }`}>
        <div className="px-4 py-3">
          <div className="flex items-center justify-between">
            {/* Left: Pair Selector & Stats */}
            <div className="flex items-center space-x-6">
              <TradingPairSelector
                pairs={mockTradingPairs}
                selectedPair={selectedPair}
                onPairChange={handlePairChange}
                currentPrice={mockMarketData?.price}
                priceChange={priceChange}
              />

              <MarketStats
                marketData={mockMarketData}
                tradingPair={currentPair}
                className="hidden lg:flex"
              />
            </div>

            {/* Right: Controls */}
            <div className="flex items-center space-x-3">
              {/* View Controls */}
              <div className="flex items-center space-x-1">
                <button
                  onClick={() => setShowOrderHistory(!showOrderHistory)}
                  className={`p-2 rounded-lg transition-colors ${
                    showOrderHistory
                      ? 'bg-primary-500 text-white'
                      : isDarkMode
                        ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                        : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                  }`}
                  title="Toggle Order History"
                >
                  <ClockIcon className="w-4 h-4" />
                </button>

                <button
                  onClick={() => setShowPositions(!showPositions)}
                  className={`p-2 rounded-lg transition-colors ${
                    showPositions
                      ? 'bg-primary-500 text-white'
                      : isDarkMode
                        ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                        : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                  }`}
                  title="Toggle Positions"
                >
                  <CurrencyDollarIcon className="w-4 h-4" />
                </button>

                <button
                  onClick={toggleFullscreen}
                  className={`p-2 rounded-lg transition-colors ${
                    isDarkMode
                      ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                      : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                  }`}
                  title="Toggle Fullscreen"
                >
                  {isFullscreen ? (
                    <EyeSlashIcon className="w-4 h-4" />
                  ) : (
                    <EyeIcon className="w-4 h-4" />
                  )}
                </button>
              </div>

              {/* Settings */}
              <button
                className={`p-2 rounded-lg transition-colors ${
                  isDarkMode
                    ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
                title="Trading Settings"
              >
                <Cog6ToothIcon className="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Main Trading Interface */}
      <div className={`${isFullscreen ? 'p-2' : 'p-4'} space-y-4`}>
        <div className={`grid gap-4 ${
          isFullscreen
            ? 'grid-cols-1'
            : 'grid-cols-1 xl:grid-cols-12'
        }`}>
          {/* Chart Section */}
          <div className={`${
            isFullscreen
              ? 'col-span-1'
              : 'xl:col-span-8'
          } space-y-4`}>
            {/* Chart Tabs */}
            <TradingViewTabs
              activeTab={activeTab}
              onTabChange={setActiveTab}
              interval={chartInterval}
              onIntervalChange={setChartInterval}
            />

            {/* Chart Content */}
            <div className={`${
              isDarkMode
                ? 'bg-dark-900 border-dark-800'
                : 'bg-white border-gray-200'
            } border rounded-xl overflow-hidden ${
              isFullscreen ? 'h-[calc(100vh-200px)]' : 'h-[500px] lg:h-[600px]'
            }`}>
              {activeTab === 'chart' && (
                <TradingChart
                  symbol={selectedPair}
                  interval={chartInterval}
                  marketData={mockMarketData}
                  className="h-full"
                />
              )}

              {activeTab === 'depth' && (
                <OrderBook
                  symbol={selectedPair}
                  mode="depth"
                  className="h-full"
                />
              )}

              {activeTab === 'trades' && (
                <RecentTrades
                  symbol={selectedPair}
                  className="h-full"
                />
              )}
            </div>
          </div>

          {/* Right Panel - Order Book & Trading */}
          {!isFullscreen && (
            <div className="xl:col-span-4 space-y-4">
              {/* Order Book */}
              <div className={`${
                isDarkMode
                  ? 'bg-dark-900 border-dark-800'
                  : 'bg-white border-gray-200'
              } border rounded-xl overflow-hidden h-[300px]`}>
                <OrderBook
                  symbol={selectedPair}
                  mode="compact"
                  className="h-full"
                />
              </div>

              {/* Order Form */}
              <div className={`${
                isDarkMode
                  ? 'bg-dark-900 border-dark-800'
                  : 'bg-white border-gray-200'
              } border rounded-xl overflow-hidden`}>
                <OrderForm
                  symbol={selectedPair}
                  marketData={mockMarketData}
                  tradingPair={currentPair}
                  orderType={orderType}
                  onOrderTypeChange={setOrderType}
                />
              </div>
            </div>
          )}
        </div>

        {/* Bottom Panels */}
        {!isFullscreen && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {/* Order History */}
            {showOrderHistory && (
              <div className={`${
                isDarkMode
                  ? 'bg-dark-900 border-dark-800'
                  : 'bg-white border-gray-200'
              } border rounded-xl overflow-hidden h-[300px]`}>
                <OrderHistory className="h-full" />
              </div>
            )}

            {/* Positions */}
            {showPositions && (
              <div className={`${
                isDarkMode
                  ? 'bg-dark-900 border-dark-800'
                  : 'bg-white border-gray-200'
              } border rounded-xl overflow-hidden h-[300px]`}>
                <PositionPanel className="h-full" />
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}

export default Trading
