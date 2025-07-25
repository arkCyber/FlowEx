/**
 * FlowEx Frontend Type Definitions
 * 
 * Comprehensive TypeScript types for the FlowEx trading platform
 * ensuring type safety across the entire application.
 */

// ============================================================================
// API Response Types
// ============================================================================

export interface ApiResponse<T = any> {
  success: boolean
  data?: T
  message?: string
  error?: string
  timestamp: string
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

// ============================================================================
// User and Authentication Types
// ============================================================================

export interface User {
  id: string
  email: string
  firstName: string
  lastName: string
  isVerified: boolean
  isActive: boolean
  twoFactorEnabled: boolean
  roles: string[]
  permissions: string[]
  createdAt: string
  updatedAt: string
  lastLoginAt?: string
}

export interface AuthState {
  isAuthenticated: boolean
  user: User | null
  token: string | null
  refreshToken: string | null
  loading: boolean
  error: string | null
}

export interface LoginRequest {
  email: string
  password: string
  rememberMe?: boolean
}

export interface RegisterRequest {
  email: string
  password: string
  firstName: string
  lastName: string
}

export interface LoginResponse {
  user: User
  token: string
  refreshToken: string
  expiresIn: number
}

// ============================================================================
// Trading Types
// ============================================================================

export interface TradingPair {
  id: string
  symbol: string
  baseAsset: string
  quoteAsset: string
  status: 'active' | 'inactive' | 'delisted'
  minOrderSize: string
  maxOrderSize: string
  pricePrecision: number
  quantityPrecision: number
  makerFee: string
  takerFee: string
  createdAt: string
  updatedAt: string
}

export type OrderSide = 'buy' | 'sell'
export type OrderType = 'market' | 'limit' | 'stop_loss' | 'take_profit'
export type OrderStatus = 'new' | 'partially_filled' | 'filled' | 'cancelled' | 'rejected' | 'expired'
export type TimeInForce = 'GTC' | 'IOC' | 'FOK'

export interface Order {
  id: string
  userId: string
  tradingPairId: string
  symbol: string
  side: OrderSide
  type: OrderType
  status: OrderStatus
  price?: string
  quantity: string
  filledQuantity: string
  remainingQuantity: string
  averagePrice?: string
  totalValue?: string
  fee: string
  feeCurrency: string
  timeInForce: TimeInForce
  stopPrice?: string
  clientOrderId?: string
  createdAt: string
  updatedAt: string
  filledAt?: string
  cancelledAt?: string
}

export interface CreateOrderRequest {
  tradingPair: string
  side: OrderSide
  type: OrderType
  price?: string
  quantity: string
  timeInForce?: TimeInForce
  stopPrice?: string
  clientOrderId?: string
}

export interface Trade {
  id: string
  tradingPairId: string
  symbol: string
  buyerOrderId: string
  sellerOrderId: string
  buyerUserId: string
  sellerUserId: string
  price: string
  quantity: string
  totalValue: string
  buyerFee: string
  sellerFee: string
  feeCurrency: string
  createdAt: string
}

// ============================================================================
// Market Data Types
// ============================================================================

export interface Ticker {
  symbol: string
  lastPrice: string
  bidPrice: string
  askPrice: string
  volume24h: string
  high24h: string
  low24h: string
  priceChange24h: string
  priceChangePercent24h: string
  updatedAt: string
}

export interface OrderBookLevel {
  price: string
  quantity: string
  orderCount: number
}

export interface OrderBook {
  symbol: string
  bids: OrderBookLevel[]
  asks: OrderBookLevel[]
  timestamp: string
}

export interface Candle {
  timestamp: number
  open: string
  high: string
  low: string
  close: string
  volume: string
}

export type CandleInterval = '1m' | '5m' | '15m' | '30m' | '1h' | '4h' | '1d' | '1w' | '1M'

// ============================================================================
// Wallet Types
// ============================================================================

export interface Balance {
  currency: string
  available: string
  locked: string
  total: string
}

export interface Transaction {
  id: string
  userId: string
  walletId: string
  type: 'deposit' | 'withdrawal' | 'trade' | 'fee'
  currency: string
  amount: string
  fee: string
  status: 'pending' | 'completed' | 'failed' | 'cancelled'
  referenceId?: string
  externalTxId?: string
  blockchainConfirmations: number
  requiredConfirmations: number
  notes?: string
  createdAt: string
  updatedAt: string
  completedAt?: string
}

// ============================================================================
// WebSocket Types
// ============================================================================

export interface WebSocketMessage {
  type: string
  data: any
  timestamp: string
}

export interface TickerUpdate {
  symbol: string
  price: string
  change: string
  changePercent: string
  volume: string
  timestamp: string
}

export interface OrderBookUpdate {
  symbol: string
  bids: OrderBookLevel[]
  asks: OrderBookLevel[]
  timestamp: string
}

export interface TradeUpdate {
  symbol: string
  price: string
  quantity: string
  side: OrderSide
  timestamp: string
}

export interface OrderUpdate {
  orderId: string
  status: OrderStatus
  filledQuantity: string
  remainingQuantity: string
  averagePrice?: string
  timestamp: string
}

// ============================================================================
// UI State Types
// ============================================================================

export interface ThemeState {
  mode: 'light' | 'dark'
  primaryColor: string
  fontSize: 'small' | 'medium' | 'large'
}

export interface UIState {
  theme: ThemeState
  sidebar: {
    isOpen: boolean
    isCollapsed: boolean
  }
  notifications: Notification[]
  loading: {
    [key: string]: boolean
  }
}

export interface Notification {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  title: string
  message: string
  timestamp: string
  read: boolean
  actions?: NotificationAction[]
}

export interface NotificationAction {
  label: string
  action: () => void
}

// ============================================================================
// Chart Types
// ============================================================================

export interface ChartData {
  timestamp: number
  value: number
  label?: string
}

export interface PriceChartData {
  timestamp: number
  open: number
  high: number
  low: number
  close: number
  volume: number
}

// ============================================================================
// Error Types
// ============================================================================

export interface AppError {
  code: string
  message: string
  details?: any
  timestamp: string
}

export interface ValidationError {
  field: string
  message: string
}

// ============================================================================
// Configuration Types
// ============================================================================

export interface AppConfig {
  apiBaseUrl: string
  wsBaseUrl: string
  appVersion: string
  buildTime: string
  environment: 'development' | 'staging' | 'production'
  features: {
    trading: boolean
    advancedCharts: boolean
    notifications: boolean
    twoFactor: boolean
  }
}

// ============================================================================
// Utility Types
// ============================================================================

export type LoadingState = 'idle' | 'loading' | 'success' | 'error'

export interface AsyncState<T> {
  data: T | null
  loading: boolean
  error: string | null
  lastUpdated?: string
}

export type SortDirection = 'asc' | 'desc'

export interface SortConfig {
  field: string
  direction: SortDirection
}

export interface FilterConfig {
  [key: string]: any
}

// ============================================================================
// Component Props Types
// ============================================================================

export interface BaseComponentProps {
  className?: string
  children?: React.ReactNode
  'data-testid'?: string
}

export interface TableColumn<T = any> {
  key: string
  label: string
  sortable?: boolean
  width?: string | number
  align?: 'left' | 'center' | 'right'
  render?: (value: any, row: T) => React.ReactNode
}

export interface TableProps<T = any> {
  data: T[]
  columns: TableColumn<T>[]
  loading?: boolean
  sortConfig?: SortConfig
  onSort?: (field: string) => void
  onRowClick?: (row: T) => void
  pagination?: {
    page: number
    limit: number
    total: number
    onPageChange: (page: number) => void
    onLimitChange: (limit: number) => void
  }
}

// ============================================================================
// Global Window Extensions
// ============================================================================

declare global {
  interface Window {
    __FLOWEX_CONFIG__: AppConfig
    gtag?: (...args: any[]) => void
  }
}
