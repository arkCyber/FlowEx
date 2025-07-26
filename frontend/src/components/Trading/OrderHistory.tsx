/**
 * Order History Component
 * 
 * Displays user's order history with filtering and status tracking
 * Shows open orders, order history, and trade history
 */

import React, { useState, useMemo } from 'react'
import { useTheme } from '../../hooks/useTheme'
import { 
  ClockIcon, 
  CheckCircleIcon, 
  XCircleIcon,
  ExclamationCircleIcon,
  ArrowPathIcon
} from '@heroicons/react/24/outline'

interface Order {
  id: string
  symbol: string
  side: 'buy' | 'sell'
  type: 'market' | 'limit' | 'stop'
  quantity: number
  price?: number
  filled: number
  status: 'new' | 'partially_filled' | 'filled' | 'cancelled' | 'rejected'
  timestamp: string
  total: number
}

interface OrderHistoryProps {
  className?: string
}

// Generate mock order data
const generateMockOrders = (): Order[] => {
  const symbols = ['BTCUSDT', 'ETHUSDT', 'ADAUSDT', 'DOTUSDT']
  const statuses: Order['status'][] = ['new', 'partially_filled', 'filled', 'cancelled', 'rejected']
  const orders: Order[] = []
  
  for (let i = 0; i < 20; i++) {
    const symbol = symbols[Math.floor(Math.random() * symbols.length)]
    const side = Math.random() > 0.5 ? 'buy' : 'sell'
    const type = Math.random() > 0.7 ? 'market' : 'limit'
    const quantity = Math.random() * 2 + 0.001
    const price = type === 'market' ? undefined : Math.random() * 50000 + 1000
    const status = statuses[Math.floor(Math.random() * statuses.length)]
    const filled = status === 'filled' ? quantity : 
                  status === 'partially_filled' ? quantity * Math.random() : 0
    
    orders.push({
      id: `order-${i}`,
      symbol,
      side,
      type,
      quantity: parseFloat(quantity.toFixed(6)),
      price: price ? parseFloat(price.toFixed(2)) : undefined,
      filled: parseFloat(filled.toFixed(6)),
      status,
      timestamp: new Date(Date.now() - i * 1000 * 60 * Math.random() * 60).toISOString(),
      total: parseFloat(((price || 45000) * quantity).toFixed(2))
    })
  }
  
  return orders.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
}

// Status Badge Component
const StatusBadge: React.FC<{ status: Order['status'] }> = ({ status }) => {
  const { isDarkMode } = useTheme()
  
  const statusConfig = {
    new: {
      icon: ClockIcon,
      color: 'text-blue-500',
      bg: isDarkMode ? 'bg-blue-500/10' : 'bg-blue-50',
      label: 'New'
    },
    partially_filled: {
      icon: ArrowPathIcon,
      color: 'text-yellow-500',
      bg: isDarkMode ? 'bg-yellow-500/10' : 'bg-yellow-50',
      label: 'Partial'
    },
    filled: {
      icon: CheckCircleIcon,
      color: 'text-success-500',
      bg: isDarkMode ? 'bg-success-500/10' : 'bg-success-50',
      label: 'Filled'
    },
    cancelled: {
      icon: XCircleIcon,
      color: 'text-gray-500',
      bg: isDarkMode ? 'bg-gray-500/10' : 'bg-gray-50',
      label: 'Cancelled'
    },
    rejected: {
      icon: ExclamationCircleIcon,
      color: 'text-danger-500',
      bg: isDarkMode ? 'bg-danger-500/10' : 'bg-danger-50',
      label: 'Rejected'
    }
  }
  
  const config = statusConfig[status]
  const Icon = config.icon
  
  return (
    <div className={`flex items-center space-x-1 px-2 py-1 rounded-full ${config.bg}`}>
      <Icon className={`w-3 h-3 ${config.color}`} />
      <span className={`text-xs font-medium ${config.color}`}>
        {config.label}
      </span>
    </div>
  )
}

// Order Row Component
const OrderRow: React.FC<{
  order: Order
  onCancel?: (orderId: string) => void
}> = ({ order, onCancel }) => {
  const { isDarkMode } = useTheme()
  const fillPercentage = (order.filled / order.quantity) * 100
  
  return (
    <div className={`p-3 border-b transition-colors ${
      isDarkMode 
        ? 'border-dark-800 hover:bg-dark-800/30' 
        : 'border-gray-200 hover:bg-gray-50'
    }`}>
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center space-x-3">
          <span className="font-medium text-sm">{order.symbol}</span>
          <span className={`text-xs px-2 py-1 rounded ${
            order.side === 'buy' 
              ? 'bg-success-500/10 text-success-500' 
              : 'bg-danger-500/10 text-danger-500'
          }`}>
            {order.side.toUpperCase()}
          </span>
          <span className={`text-xs ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            {order.type.toUpperCase()}
          </span>
        </div>
        
        <div className="flex items-center space-x-2">
          <StatusBadge status={order.status} />
          {(order.status === 'new' || order.status === 'partially_filled') && onCancel && (
            <button
              onClick={() => onCancel(order.id)}
              className={`text-xs px-2 py-1 rounded transition-colors ${
                isDarkMode
                  ? 'bg-danger-500/10 text-danger-400 hover:bg-danger-500/20'
                  : 'bg-danger-50 text-danger-600 hover:bg-danger-100'
              }`}
            >
              Cancel
            </button>
          )}
        </div>
      </div>
      
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 text-xs">
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Price
          </div>
          <div className="font-mono">
            {order.price ? `$${order.price.toFixed(2)}` : 'Market'}
          </div>
        </div>
        
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Amount
          </div>
          <div className="font-mono">
            {order.quantity.toFixed(6)}
          </div>
        </div>
        
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Filled
          </div>
          <div className="space-y-1">
            <div className="font-mono">
              {order.filled.toFixed(6)} ({fillPercentage.toFixed(1)}%)
            </div>
            {fillPercentage > 0 && (
              <div className="w-full bg-gray-200 rounded-full h-1">
                <div 
                  className="bg-primary-500 h-1 rounded-full transition-all"
                  style={{ width: `${fillPercentage}%` }}
                />
              </div>
            )}
          </div>
        </div>
        
        <div>
          <div className={`${isDarkMode ? 'text-dark-400' : 'text-gray-500'}`}>
            Time
          </div>
          <div className="font-mono">
            {new Date(order.timestamp).toLocaleString([], {
              month: 'short',
              day: '2-digit',
              hour: '2-digit',
              minute: '2-digit'
            })}
          </div>
        </div>
      </div>
    </div>
  )
}

export const OrderHistory: React.FC<OrderHistoryProps> = ({
  className = ''
}) => {
  const { isDarkMode } = useTheme()
  const [activeTab, setActiveTab] = useState<'open' | 'history'>('open')
  const [statusFilter, setStatusFilter] = useState<'all' | Order['status']>('all')
  
  const orders = useMemo(() => generateMockOrders(), [])
  
  const filteredOrders = useMemo(() => {
    let filtered = orders
    
    // Filter by tab
    if (activeTab === 'open') {
      filtered = filtered.filter(order => 
        order.status === 'new' || order.status === 'partially_filled'
      )
    } else {
      filtered = filtered.filter(order => 
        order.status === 'filled' || order.status === 'cancelled' || order.status === 'rejected'
      )
    }
    
    // Filter by status
    if (statusFilter !== 'all') {
      filtered = filtered.filter(order => order.status === statusFilter)
    }
    
    return filtered
  }, [orders, activeTab, statusFilter])
  
  const handleCancelOrder = (orderId: string) => {
    console.log('Cancelling order:', orderId)
    // Here you would call the API to cancel the order
  }
  
  return (
    <div className={`${className} flex flex-col`}>
      {/* Header */}
      <div className={`p-4 border-b ${
        isDarkMode ? 'border-dark-800' : 'border-gray-200'
      }`}>
        <div className="flex items-center justify-between">
          <h3 className="font-semibold">Orders</h3>
          
          {/* Status Filter */}
          <select
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value as any)}
            className={`text-xs px-2 py-1 rounded border ${
              isDarkMode
                ? 'bg-dark-800 border-dark-700 text-dark-200'
                : 'bg-white border-gray-200 text-gray-700'
            } focus:outline-none focus:ring-2 focus:ring-primary-500/20`}
          >
            <option value="all">All Status</option>
            <option value="new">New</option>
            <option value="partially_filled">Partial</option>
            <option value="filled">Filled</option>
            <option value="cancelled">Cancelled</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>
        
        {/* Tabs */}
        <div className={`flex mt-3 rounded-lg p-1 ${
          isDarkMode ? 'bg-dark-800' : 'bg-gray-100'
        }`}>
          {[
            { key: 'open', label: 'Open Orders' },
            { key: 'history', label: 'Order History' }
          ].map(tab => (
            <button
              key={tab.key}
              onClick={() => setActiveTab(tab.key as any)}
              className={`flex-1 py-2 text-sm font-medium rounded-md transition-colors ${
                activeTab === tab.key
                  ? isDarkMode
                    ? 'bg-dark-700 text-primary-400'
                    : 'bg-white text-primary-600 shadow-sm'
                  : isDarkMode
                    ? 'text-dark-300 hover:text-dark-100'
                    : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>
      </div>
      
      {/* Orders List */}
      <div className="flex-1 overflow-y-auto">
        {filteredOrders.length === 0 ? (
          <div className={`p-8 text-center ${
            isDarkMode ? 'text-dark-400' : 'text-gray-500'
          }`}>
            <ClockIcon className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div className="text-sm">No orders found</div>
            <div className="text-xs mt-1">
              {activeTab === 'open' ? 'No open orders' : 'No order history'}
            </div>
          </div>
        ) : (
          filteredOrders.map(order => (
            <OrderRow
              key={order.id}
              order={order}
              onCancel={activeTab === 'open' ? handleCancelOrder : undefined}
            />
          ))
        )}
      </div>
    </div>
  )
}
