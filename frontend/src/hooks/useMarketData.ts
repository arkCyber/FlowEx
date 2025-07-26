/**
 * Market Data Hook
 * 
 * Fetches and manages market data for trading pairs
 * Provides real-time price updates and market statistics
 */

import { useState, useEffect, useMemo } from 'react'

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

interface UseMarketDataResult {
  marketData: MarketData | null
  isLoading: boolean
  error: string | null
  refetch: () => void
}

// Mock market data generator
const generateMockMarketData = (symbol: string): MarketData => {
  const basePrice = symbol === 'BTCUSDT' ? 45000 : 
                   symbol === 'ETHUSDT' ? 2800 : 
                   symbol === 'ADAUSDT' ? 0.45 : 
                   symbol === 'DOTUSDT' ? 12.34 : 1000
  
  const priceVariation = (Math.random() - 0.5) * basePrice * 0.02 // ±1% variation
  const currentPrice = basePrice + priceVariation
  
  const change = (Math.random() - 0.5) * basePrice * 0.05 // ±2.5% change
  const changePercent = (change / basePrice) * 100
  
  const high = currentPrice + Math.random() * basePrice * 0.03
  const low = currentPrice - Math.random() * basePrice * 0.03
  const volume = Math.random() * 10000 + 1000
  
  return {
    symbol,
    price: currentPrice.toLocaleString('en-US', { 
      minimumFractionDigits: 2, 
      maximumFractionDigits: symbol.includes('USDT') ? 2 : 6 
    }),
    change: change.toLocaleString('en-US', { 
      minimumFractionDigits: 2, 
      maximumFractionDigits: 2 
    }),
    change_percent: changePercent.toFixed(2),
    high: high.toLocaleString('en-US', { 
      minimumFractionDigits: 2, 
      maximumFractionDigits: symbol.includes('USDT') ? 2 : 6 
    }),
    low: low.toLocaleString('en-US', { 
      minimumFractionDigits: 2, 
      maximumFractionDigits: symbol.includes('USDT') ? 2 : 6 
    }),
    volume: volume.toLocaleString('en-US', { 
      minimumFractionDigits: 2, 
      maximumFractionDigits: 2 
    }),
    timestamp: new Date().toISOString()
  }
}

export const useMarketData = (symbol: string): UseMarketDataResult => {
  const [marketData, setMarketData] = useState<MarketData | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  
  // Simulate API call
  const fetchMarketData = useMemo(() => {
    return async () => {
      try {
        setIsLoading(true)
        setError(null)
        
        // Simulate network delay
        await new Promise(resolve => setTimeout(resolve, 500))
        
        // Generate mock data
        const data = generateMockMarketData(symbol)
        setMarketData(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch market data')
      } finally {
        setIsLoading(false)
      }
    }
  }, [symbol])
  
  // Fetch data on symbol change
  useEffect(() => {
    fetchMarketData()
  }, [fetchMarketData])
  
  // Simulate real-time updates
  useEffect(() => {
    if (!marketData) return
    
    const interval = setInterval(() => {
      const updatedData = generateMockMarketData(symbol)
      setMarketData(updatedData)
    }, 5000) // Update every 5 seconds
    
    return () => clearInterval(interval)
  }, [symbol, marketData])
  
  const refetch = () => {
    fetchMarketData()
  }
  
  return {
    marketData,
    isLoading,
    error,
    refetch
  }
}
