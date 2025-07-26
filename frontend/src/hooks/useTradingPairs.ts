/**
 * Trading Pairs Hook
 * 
 * Fetches and manages available trading pairs
 * Provides filtering and search functionality
 */

import { useState, useEffect, useMemo } from 'react'

interface TradingPair {
  symbol: string
  base_asset: string
  quote_asset: string
  status: string
  min_price?: string
  max_price?: string
  min_qty?: string
  max_qty?: string
  step_size?: string
  tick_size?: string
}

interface UseTradingPairsResult {
  tradingPairs: TradingPair[] | null
  isLoading: boolean
  error: string | null
  refetch: () => void
}

// Mock trading pairs data
const mockTradingPairs: TradingPair[] = [
  {
    symbol: 'BTCUSDT',
    base_asset: 'BTC',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.01',
    max_price: '1000000.00',
    min_qty: '0.00001',
    max_qty: '9000.00000000',
    step_size: '0.00001',
    tick_size: '0.01'
  },
  {
    symbol: 'ETHUSDT',
    base_asset: 'ETH',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.01',
    max_price: '100000.00',
    min_qty: '0.0001',
    max_qty: '100000.00000000',
    step_size: '0.0001',
    tick_size: '0.01'
  },
  {
    symbol: 'ADAUSDT',
    base_asset: 'ADA',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.0001',
    max_price: '1000.0000',
    min_qty: '0.1',
    max_qty: '90000000.0',
    step_size: '0.1',
    tick_size: '0.0001'
  },
  {
    symbol: 'DOTUSDT',
    base_asset: 'DOT',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  {
    symbol: 'LINKUSDT',
    base_asset: 'LINK',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  {
    symbol: 'UNIUSDT',
    base_asset: 'UNI',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  {
    symbol: 'SOLUSDT',
    base_asset: 'SOL',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  {
    symbol: 'MATICUSDT',
    base_asset: 'MATIC',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.0001',
    max_price: '1000.0000',
    min_qty: '0.1',
    max_qty: '90000000.0',
    step_size: '0.1',
    tick_size: '0.0001'
  },
  {
    symbol: 'AVAXUSDT',
    base_asset: 'AVAX',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  {
    symbol: 'ATOMUSDT',
    base_asset: 'ATOM',
    quote_asset: 'USDT',
    status: 'TRADING',
    min_price: '0.001',
    max_price: '10000.000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.001'
  },
  // BTC pairs
  {
    symbol: 'ETHBTC',
    base_asset: 'ETH',
    quote_asset: 'BTC',
    status: 'TRADING',
    min_price: '0.000001',
    max_price: '1000.000000',
    min_qty: '0.001',
    max_qty: '100000.000',
    step_size: '0.001',
    tick_size: '0.000001'
  },
  {
    symbol: 'ADABTC',
    base_asset: 'ADA',
    quote_asset: 'BTC',
    status: 'TRADING',
    min_price: '0.00000001',
    max_price: '1.00000000',
    min_qty: '1',
    max_qty: '90000000',
    step_size: '1',
    tick_size: '0.00000001'
  },
  {
    symbol: 'DOTBTC',
    base_asset: 'DOT',
    quote_asset: 'BTC',
    status: 'TRADING',
    min_price: '0.00000001',
    max_price: '1.00000000',
    min_qty: '0.01',
    max_qty: '9000000.00',
    step_size: '0.01',
    tick_size: '0.00000001'
  }
]

export const useTradingPairs = (): UseTradingPairsResult => {
  const [tradingPairs, setTradingPairs] = useState<TradingPair[] | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  
  // Simulate API call
  const fetchTradingPairs = useMemo(() => {
    return async () => {
      try {
        setIsLoading(true)
        setError(null)
        
        // Simulate network delay
        await new Promise(resolve => setTimeout(resolve, 300))
        
        // Return mock data
        setTradingPairs(mockTradingPairs)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch trading pairs')
      } finally {
        setIsLoading(false)
      }
    }
  }, [])
  
  // Fetch data on mount
  useEffect(() => {
    fetchTradingPairs()
  }, [fetchTradingPairs])
  
  const refetch = () => {
    fetchTradingPairs()
  }
  
  return {
    tradingPairs,
    isLoading,
    error,
    refetch
  }
}
