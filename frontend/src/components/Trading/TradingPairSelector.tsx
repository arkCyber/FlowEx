/**
 * Trading Pair Selector Component
 * 
 * Professional trading pair selector with search, favorites, and price display
 * Inspired by Binance and OKX interfaces
 */

import React, { useState, useMemo, useRef, useEffect } from 'react'
import { ChevronDownIcon, MagnifyingGlassIcon, StarIcon } from '@heroicons/react/24/outline'
import { StarIcon as StarSolidIcon } from '@heroicons/react/24/solid'
import { useTheme } from '../../hooks/useTheme'

interface TradingPair {
  symbol: string
  base_asset: string
  quote_asset: string
  status: string
}

interface PriceChange {
  value: number
  percentage: number
  isPositive: boolean
}

interface TradingPairSelectorProps {
  pairs: TradingPair[]
  selectedPair: string
  onPairChange: (pair: string) => void
  currentPrice?: string
  priceChange?: PriceChange
  className?: string
}

export const TradingPairSelector: React.FC<TradingPairSelectorProps> = ({
  pairs,
  selectedPair,
  onPairChange,
  currentPrice,
  priceChange,
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [isOpen, setIsOpen] = useState(false)
  const [searchTerm, setSearchTerm] = useState('')
  const [activeTab, setActiveTab] = useState<'all' | 'favorites' | 'usdt' | 'btc'>('all')
  const [favorites, setFavorites] = useState<Set<string>>(new Set(['BTCUSDT', 'ETHUSDT']))
  const dropdownRef = useRef<HTMLDivElement>(null)
  
  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false)
      }
    }
    
    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])
  
  // Filter pairs based on search and tab
  const filteredPairs = useMemo(() => {
    let filtered = pairs
    
    // Filter by tab
    if (activeTab === 'favorites') {
      filtered = filtered.filter(pair => favorites.has(pair.symbol))
    } else if (activeTab === 'usdt') {
      filtered = filtered.filter(pair => pair.quote_asset === 'USDT')
    } else if (activeTab === 'btc') {
      filtered = filtered.filter(pair => pair.quote_asset === 'BTC')
    }
    
    // Filter by search term
    if (searchTerm) {
      filtered = filtered.filter(pair => 
        pair.symbol.toLowerCase().includes(searchTerm.toLowerCase()) ||
        pair.base_asset.toLowerCase().includes(searchTerm.toLowerCase())
      )
    }
    
    return filtered
  }, [pairs, activeTab, favorites, searchTerm])
  
  // Toggle favorite
  const toggleFavorite = (symbol: string, e: React.MouseEvent) => {
    e.stopPropagation()
    const newFavorites = new Set(favorites)
    if (newFavorites.has(symbol)) {
      newFavorites.delete(symbol)
    } else {
      newFavorites.add(symbol)
    }
    setFavorites(newFavorites)
  }
  
  // Handle pair selection
  const handlePairSelect = (symbol: string) => {
    onPairChange(symbol)
    setIsOpen(false)
  }
  
  const currentPair = pairs.find(pair => pair.symbol === selectedPair)
  
  return (
    <div className={`relative ${className}`} ref={dropdownRef}>
      {/* Selected Pair Display */}
      <button
        onClick={() => setIsOpen(!isOpen)}
        className={`flex items-center space-x-3 px-4 py-2 rounded-lg transition-colors ${
          isDarkMode
            ? 'bg-dark-800 hover:bg-dark-700 text-dark-50'
            : 'bg-gray-100 hover:bg-gray-200 text-gray-900'
        }`}
      >
        {/* Pair Info */}
        <div className="flex items-center space-x-2">
          <div className="text-left">
            <div className="flex items-center space-x-1">
              <span className="font-semibold text-lg">{currentPair?.base_asset}</span>
              <span className={`text-sm ${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
                /{currentPair?.quote_asset}
              </span>
            </div>
            
            {/* Price and Change */}
            {currentPrice && priceChange && (
              <div className="flex items-center space-x-2 text-sm">
                <span className="font-mono">${currentPrice}</span>
                <span className={`flex items-center space-x-1 ${
                  priceChange.isPositive ? 'text-success-500' : 'text-danger-500'
                }`}>
                  <span>{priceChange.isPositive ? '+' : ''}{priceChange.percentage.toFixed(2)}%</span>
                </span>
              </div>
            )}
          </div>
        </div>
        
        <ChevronDownIcon className={`w-4 h-4 transition-transform ${
          isOpen ? 'rotate-180' : ''
        }`} />
      </button>
      
      {/* Dropdown */}
      {isOpen && (
        <div className={`absolute top-full left-0 mt-2 w-96 rounded-xl shadow-2xl border z-50 ${
          isDarkMode
            ? 'bg-dark-900 border-dark-700'
            : 'bg-white border-gray-200'
        }`}>
          {/* Search */}
          <div className="p-4 border-b border-dark-800">
            <div className="relative">
              <MagnifyingGlassIcon className={`absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 ${
                isDarkMode ? 'text-dark-400' : 'text-gray-400'
              }`} />
              <input
                type="text"
                placeholder="Search pairs..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className={`w-full pl-10 pr-4 py-2 rounded-lg border transition-colors ${
                  isDarkMode
                    ? 'bg-dark-800 border-dark-700 text-dark-50 placeholder-dark-400 focus:border-primary-500'
                    : 'bg-gray-50 border-gray-200 text-gray-900 placeholder-gray-400 focus:border-primary-500'
                } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
              />
            </div>
          </div>
          
          {/* Tabs */}
          <div className={`flex border-b ${isDarkMode ? 'border-dark-800' : 'border-gray-200'}`}>
            {[
              { key: 'all', label: 'All' },
              { key: 'favorites', label: 'Favorites' },
              { key: 'usdt', label: 'USDT' },
              { key: 'btc', label: 'BTC' }
            ].map(tab => (
              <button
                key={tab.key}
                onClick={() => setActiveTab(tab.key as any)}
                className={`flex-1 px-4 py-3 text-sm font-medium transition-colors ${
                  activeTab === tab.key
                    ? isDarkMode
                      ? 'text-primary-400 border-b-2 border-primary-400'
                      : 'text-primary-600 border-b-2 border-primary-600'
                    : isDarkMode
                      ? 'text-dark-400 hover:text-dark-200'
                      : 'text-gray-500 hover:text-gray-700'
                }`}
              >
                {tab.label}
                {tab.key === 'favorites' && favorites.size > 0 && (
                  <span className={`ml-1 text-xs px-1.5 py-0.5 rounded-full ${
                    isDarkMode ? 'bg-dark-700 text-dark-300' : 'bg-gray-100 text-gray-600'
                  }`}>
                    {favorites.size}
                  </span>
                )}
              </button>
            ))}
          </div>
          
          {/* Pair List */}
          <div className="max-h-80 overflow-y-auto">
            {filteredPairs.length === 0 ? (
              <div className={`p-8 text-center ${
                isDarkMode ? 'text-dark-400' : 'text-gray-500'
              }`}>
                <div className="text-sm">No trading pairs found</div>
                {searchTerm && (
                  <div className="text-xs mt-1">Try adjusting your search terms</div>
                )}
              </div>
            ) : (
              filteredPairs.map(pair => (
                <button
                  key={pair.symbol}
                  onClick={() => handlePairSelect(pair.symbol)}
                  className={`w-full flex items-center justify-between px-4 py-3 transition-colors ${
                    pair.symbol === selectedPair
                      ? isDarkMode
                        ? 'bg-primary-500/10 text-primary-400'
                        : 'bg-primary-50 text-primary-600'
                      : isDarkMode
                        ? 'hover:bg-dark-800 text-dark-50'
                        : 'hover:bg-gray-50 text-gray-900'
                  }`}
                >
                  <div className="flex items-center space-x-3">
                    {/* Favorite Star */}
                    <button
                      onClick={(e) => toggleFavorite(pair.symbol, e)}
                      className={`p-1 rounded transition-colors ${
                        favorites.has(pair.symbol)
                          ? 'text-yellow-500 hover:text-yellow-400'
                          : isDarkMode
                            ? 'text-dark-500 hover:text-dark-300'
                            : 'text-gray-300 hover:text-gray-500'
                      }`}
                    >
                      {favorites.has(pair.symbol) ? (
                        <StarSolidIcon className="w-4 h-4" />
                      ) : (
                        <StarIcon className="w-4 h-4" />
                      )}
                    </button>
                    
                    {/* Pair Info */}
                    <div className="text-left">
                      <div className="font-medium">
                        {pair.base_asset}
                        <span className={`ml-1 text-sm ${
                          isDarkMode ? 'text-dark-400' : 'text-gray-500'
                        }`}>
                          /{pair.quote_asset}
                        </span>
                      </div>
                    </div>
                  </div>
                  
                  {/* Mock Price Data */}
                  <div className="text-right text-sm">
                    <div className="font-mono">
                      {pair.symbol === 'BTCUSDT' ? '$45,234.56' :
                       pair.symbol === 'ETHUSDT' ? '$2,834.12' :
                       pair.symbol === 'ADAUSDT' ? '$0.4567' :
                       '$12.34'}
                    </div>
                    <div className={`text-xs ${
                      Math.random() > 0.5 ? 'text-success-500' : 'text-danger-500'
                    }`}>
                      {Math.random() > 0.5 ? '+' : '-'}{(Math.random() * 5).toFixed(2)}%
                    </div>
                  </div>
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  )
}
