/**
 * FlowEx Trading API Service
 */

import api from './api'
import type { 
  TradingPair, 
  Order, 
  Trade, 
  CreateOrderRequest, 
  OrderBook,
  Ticker 
} from '../types'

export const tradingApi = {
  // Trading pairs
  getTradingPairs: (): Promise<TradingPair[]> =>
    api.get('/api/trading/pairs'),
    
  getTradingPair: (symbol: string): Promise<TradingPair> =>
    api.get(`/api/trading/pairs/${symbol}`),
    
  // Orders
  getOrders: (): Promise<Order[]> =>
    api.get('/api/trading/orders'),
    
  getOrder: (orderId: string): Promise<Order> =>
    api.get(`/api/trading/orders/${orderId}`),
    
  createOrder: (orderData: CreateOrderRequest): Promise<Order> =>
    api.post('/api/trading/orders', orderData),
    
  cancelOrder: (orderId: string): Promise<void> =>
    api.delete(`/api/trading/orders/${orderId}`),
    
  cancelAllOrders: (symbol?: string): Promise<void> =>
    api.delete('/api/trading/orders', { params: { symbol } }),
    
  // Trades
  getTrades: (symbol?: string): Promise<Trade[]> =>
    api.get('/api/trading/trades', { params: { symbol } }),
    
  getUserTrades: (): Promise<Trade[]> =>
    api.get('/api/trading/trades/user'),
    
  // Market data
  getOrderBook: (symbol: string): Promise<OrderBook> =>
    api.get(`/api/trading/orderbook/${symbol}`),
    
  getTicker: (symbol: string): Promise<Ticker> =>
    api.get(`/api/market-data/ticker/${symbol}`),
    
  getAllTickers: (): Promise<Ticker[]> =>
    api.get('/api/market-data/tickers'),
}
