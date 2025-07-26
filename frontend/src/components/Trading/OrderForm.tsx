/**
 * Professional Order Form Component
 * 
 * Advanced order placement form with multiple order types
 * Includes balance display, order validation, and quick actions
 */

import React, { useState, useMemo } from 'react'
import { useTheme } from '../../hooks/useTheme'
import { 
  CurrencyDollarIcon, 
  CalculatorIcon,
  ExclamationTriangleIcon,
  CheckCircleIcon
} from '@heroicons/react/24/outline'

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

interface OrderFormProps {
  symbol: string
  marketData?: MarketData
  tradingPair?: TradingPair
  orderType: 'buy' | 'sell'
  onOrderTypeChange: (type: 'buy' | 'sell') => void
  className?: string
}

export const OrderForm: React.FC<OrderFormProps> = ({
  symbol,
  marketData,
  tradingPair,
  orderType,
  onOrderTypeChange,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  
  // Form state
  const [orderMode, setOrderMode] = useState<'market' | 'limit' | 'stop'>('limit')
  const [price, setPrice] = useState('')
  const [quantity, setQuantity] = useState('')
  const [stopPrice, setStopPrice] = useState('')
  const [timeInForce, setTimeInForce] = useState<'GTC' | 'IOC' | 'FOK'>('GTC')
  
  // Mock balances
  const mockBalances = {
    BTC: 0.12345678,
    ETH: 2.5678,
    USDT: 10000.50,
    ADA: 1500.25,
    DOT: 100.75
  }
  
  const currentPrice = marketData ? parseFloat(marketData.price.replace(',', '')) : 0
  const baseAsset = tradingPair?.base_asset || 'BTC'
  const quoteAsset = tradingPair?.quote_asset || 'USDT'
  
  // Calculate order details
  const orderDetails = useMemo(() => {
    const orderPrice = orderMode === 'market' ? currentPrice : parseFloat(price) || 0
    const orderQuantity = parseFloat(quantity) || 0
    const total = orderPrice * orderQuantity
    
    const availableBalance = orderType === 'buy' 
      ? mockBalances[quoteAsset as keyof typeof mockBalances] || 0
      : mockBalances[baseAsset as keyof typeof mockBalances] || 0
    
    const maxQuantity = orderType === 'buy'
      ? orderPrice > 0 ? availableBalance / orderPrice : 0
      : availableBalance
    
    const fee = total * 0.001 // 0.1% fee
    const totalWithFee = orderType === 'buy' ? total + fee : total - fee
    
    return {
      orderPrice,
      orderQuantity,
      total,
      fee,
      totalWithFee,
      availableBalance,
      maxQuantity,
      isValid: orderQuantity > 0 && orderPrice > 0 && total <= availableBalance
    }
  }, [orderMode, price, quantity, currentPrice, orderType, baseAsset, quoteAsset])
  
  // Quick percentage buttons
  const percentageButtons = [25, 50, 75, 100]
  
  const handlePercentageClick = (percentage: number) => {
    const maxQty = orderDetails.maxQuantity
    const newQuantity = (maxQty * percentage / 100).toFixed(6)
    setQuantity(newQuantity)
  }
  
  const handleSubmitOrder = () => {
    if (!orderDetails.isValid) return
    
    const order = {
      symbol,
      side: orderType,
      type: orderMode,
      quantity: orderDetails.orderQuantity,
      price: orderMode === 'market' ? undefined : orderDetails.orderPrice,
      stopPrice: orderMode === 'stop' ? parseFloat(stopPrice) : undefined,
      timeInForce
    }
    
    console.log('Submitting order:', order)
    // Here you would call the API to place the order
  }
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Header */}
      <div className={`p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <div className="flex items-center space-x-1">
          <button
            onClick={() => onOrderTypeChange('buy')}
            className={`flex-1 py-2 px-4 rounded-lg font-medium transition-colors ${
              orderType === 'buy'
                ? 'bg-success-500 text-white'
                : isDarkMode
                  ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
            }`}
          >
            Buy {baseAsset}
          </button>
          <button
            onClick={() => onOrderTypeChange('sell')}
            className={`flex-1 py-2 px-4 rounded-lg font-medium transition-colors ${
              orderType === 'sell'
                ? 'bg-danger-500 text-white'
                : isDarkMode
                  ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
            }`}
          >
            Sell {baseAsset}
          </button>
        </div>
      </div>
      
      {/* Order Type Tabs */}
      <div className={`flex border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        {[
          { key: 'market', label: 'Market' },
          { key: 'limit', label: 'Limit' },
          { key: 'stop', label: 'Stop' }
        ].map(mode => (
          <button
            key={mode.key}
            onClick={() => setOrderMode(mode.key as any)}
            className={`flex-1 py-3 text-sm font-medium transition-colors ${
              orderMode === mode.key
                ? isDarkMode
                  ? 'text-primary-400 border-b-2 border-primary-400'
                  : 'text-primary-600 border-b-2 border-primary-600'
                : isDarkMode
                  ? 'text-dark-400 hover:text-dark-200'
                  : 'text-gray-500 hover:text-gray-700'
            }`}
          >
            {mode.label}
          </button>
        ))}
      </div>
      
      {/* Form Content */}
      <div className="flex-1 p-4 space-y-4">
        {/* Available Balance */}
        <div className={`p-3 rounded-lg ${
          isDarkMode ? 'bg-dark-800' : 'bg-gray-50'
        }`}>
          <div className="flex items-center justify-between text-sm">
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Available
            </span>
            <div className="flex items-center space-x-1">
              <CurrencyDollarIcon className="w-4 h-4" />
              <span className="font-mono">
                {orderDetails.availableBalance.toFixed(6)} {orderType === 'buy' ? quoteAsset : baseAsset}
              </span>
            </div>
          </div>
        </div>
        
        {/* Stop Price (for stop orders) */}
        {orderMode === 'stop' && (
          <div>
            <label className={`block text-sm font-medium mb-2 ${
              isDarkMode ? 'text-dark-200' : 'text-gray-700'
            }`}>
              Stop Price
            </label>
            <input
              type="number"
              value={stopPrice}
              onChange={(e) => setStopPrice(e.target.value)}
              placeholder="0.00"
              className={`w-full px-3 py-2 rounded-lg border font-mono ${
                isDarkMode
                  ? 'bg-dark-800 border-dark-700 text-dark-50 placeholder-dark-400'
                  : 'bg-white border-gray-200 text-gray-900 placeholder-gray-400'
              } focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500`}
            />
          </div>
        )}
        
        {/* Price (for limit orders) */}
        {orderMode !== 'market' && (
          <div>
            <label className={`block text-sm font-medium mb-2 ${
              isDarkMode ? 'text-dark-200' : 'text-gray-700'
            }`}>
              Price
            </label>
            <div className="relative">
              <input
                type="number"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                placeholder={currentPrice.toFixed(2)}
                className={`w-full px-3 py-2 pr-12 rounded-lg border font-mono ${
                  isDarkMode
                    ? 'bg-dark-800 border-dark-700 text-dark-50 placeholder-dark-400'
                    : 'bg-white border-gray-200 text-gray-900 placeholder-gray-400'
                } focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500`}
              />
              <div className={`absolute right-3 top-1/2 transform -translate-y-1/2 text-sm ${
                isDarkMode ? 'text-dark-400' : 'text-gray-500'
              }`}>
                {quoteAsset}
              </div>
            </div>
          </div>
        )}
        
        {/* Quantity */}
        <div>
          <label className={`block text-sm font-medium mb-2 ${
            isDarkMode ? 'text-dark-200' : 'text-gray-700'
          }`}>
            Amount
          </label>
          <div className="relative">
            <input
              type="number"
              value={quantity}
              onChange={(e) => setQuantity(e.target.value)}
              placeholder="0.00"
              className={`w-full px-3 py-2 pr-12 rounded-lg border font-mono ${
                isDarkMode
                  ? 'bg-dark-800 border-dark-700 text-dark-50 placeholder-dark-400'
                  : 'bg-white border-gray-200 text-gray-900 placeholder-gray-400'
              } focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500`}
            />
            <div className={`absolute right-3 top-1/2 transform -translate-y-1/2 text-sm ${
              isDarkMode ? 'text-dark-400' : 'text-gray-500'
            }`}>
              {baseAsset}
            </div>
          </div>
          
          {/* Percentage Buttons */}
          <div className="flex space-x-1 mt-2">
            {percentageButtons.map(percentage => (
              <button
                key={percentage}
                onClick={() => handlePercentageClick(percentage)}
                className={`flex-1 py-1 text-xs rounded transition-colors ${
                  isDarkMode
                    ? 'bg-dark-800 text-dark-300 hover:bg-dark-700'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                {percentage}%
              </button>
            ))}
          </div>
        </div>
        
        {/* Order Summary */}
        <div className={`p-3 rounded-lg space-y-2 ${
          isDarkMode ? 'bg-dark-800' : 'bg-gray-50'
        }`}>
          <div className="flex justify-between text-sm">
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Total
            </span>
            <span className="font-mono">
              {orderDetails.total.toFixed(2)} {quoteAsset}
            </span>
          </div>
          <div className="flex justify-between text-sm">
            <span className={isDarkMode ? 'text-dark-400' : 'text-gray-500'}>
              Fee (0.1%)
            </span>
            <span className="font-mono">
              {orderDetails.fee.toFixed(2)} {quoteAsset}
            </span>
          </div>
          <div className={`flex justify-between text-sm font-medium pt-2 border-t ${
            isDarkMode ? 'border-dark-700' : 'border-gray-200'
          }`}>
            <span>Total {orderType === 'buy' ? 'Cost' : 'Receive'}</span>
            <span className="font-mono">
              {orderDetails.totalWithFee.toFixed(2)} {quoteAsset}
            </span>
          </div>
        </div>
        
        {/* Submit Button */}
        <button
          onClick={handleSubmitOrder}
          disabled={!orderDetails.isValid}
          className={`w-full py-3 rounded-lg font-medium transition-colors ${
            orderDetails.isValid
              ? orderType === 'buy'
                ? 'bg-success-500 hover:bg-success-600 text-white'
                : 'bg-danger-500 hover:bg-danger-600 text-white'
              : isDarkMode
                ? 'bg-dark-700 text-dark-500 cursor-not-allowed'
                : 'bg-gray-200 text-gray-400 cursor-not-allowed'
          }`}
        >
          {orderType === 'buy' ? 'Buy' : 'Sell'} {baseAsset}
        </button>
        
        {/* Validation Messages */}
        {!orderDetails.isValid && orderDetails.orderQuantity > 0 && (
          <div className="flex items-center space-x-2 text-sm text-danger-500">
            <ExclamationTriangleIcon className="w-4 h-4" />
            <span>Insufficient balance</span>
          </div>
        )}
      </div>
    </div>
  )
}
