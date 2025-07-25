/**
 * FlowEx Trading Redux Slice
 * 
 * Manages trading state including orders, trades, trading pairs,
 * and order book data.
 */

import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit'
import { tradingApi } from '../../services/tradingApi'
import type { Order, Trade, TradingPair, CreateOrderRequest } from '../../types'

interface TradingState {
  orders: Order[]
  trades: Trade[]
  tradingPairs: TradingPair[]
  selectedPair: string | null
  loading: {
    orders: boolean
    trades: boolean
    pairs: boolean
    createOrder: boolean
  }
  error: string | null
}

const initialState: TradingState = {
  orders: [],
  trades: [],
  tradingPairs: [],
  selectedPair: null,
  loading: {
    orders: false,
    trades: false,
    pairs: false,
    createOrder: false,
  },
  error: null,
}

// Async thunks
export const fetchTradingPairs = createAsyncThunk(
  'trading/fetchTradingPairs',
  async (_, { rejectWithValue }) => {
    try {
      return await tradingApi.getTradingPairs()
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.message || 'Failed to fetch trading pairs')
    }
  }
)

export const fetchOrders = createAsyncThunk(
  'trading/fetchOrders',
  async (_, { rejectWithValue }) => {
    try {
      return await tradingApi.getOrders()
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.message || 'Failed to fetch orders')
    }
  }
)

export const createOrder = createAsyncThunk(
  'trading/createOrder',
  async (orderData: CreateOrderRequest, { rejectWithValue }) => {
    try {
      return await tradingApi.createOrder(orderData)
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.message || 'Failed to create order')
    }
  }
)

export const cancelOrder = createAsyncThunk(
  'trading/cancelOrder',
  async (orderId: string, { rejectWithValue }) => {
    try {
      await tradingApi.cancelOrder(orderId)
      return orderId
    } catch (error: any) {
      return rejectWithValue(error.response?.data?.message || 'Failed to cancel order')
    }
  }
)

const tradingSlice = createSlice({
  name: 'trading',
  initialState,
  reducers: {
    setSelectedPair: (state, action: PayloadAction<string>) => {
      state.selectedPair = action.payload
    },
    updateOrder: (state, action: PayloadAction<Order>) => {
      const index = state.orders.findIndex(order => order.id === action.payload.id)
      if (index !== -1) {
        state.orders[index] = action.payload
      }
    },
    addTrade: (state, action: PayloadAction<Trade>) => {
      state.trades.unshift(action.payload)
      if (state.trades.length > 100) {
        state.trades = state.trades.slice(0, 100)
      }
    },
    clearError: (state) => {
      state.error = null
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchTradingPairs.pending, (state) => {
        state.loading.pairs = true
      })
      .addCase(fetchTradingPairs.fulfilled, (state, action) => {
        state.loading.pairs = false
        state.tradingPairs = action.payload
      })
      .addCase(fetchTradingPairs.rejected, (state, action) => {
        state.loading.pairs = false
        state.error = action.payload as string
      })
      .addCase(fetchOrders.pending, (state) => {
        state.loading.orders = true
      })
      .addCase(fetchOrders.fulfilled, (state, action) => {
        state.loading.orders = false
        state.orders = action.payload
      })
      .addCase(fetchOrders.rejected, (state, action) => {
        state.loading.orders = false
        state.error = action.payload as string
      })
      .addCase(createOrder.pending, (state) => {
        state.loading.createOrder = true
      })
      .addCase(createOrder.fulfilled, (state, action) => {
        state.loading.createOrder = false
        state.orders.unshift(action.payload)
      })
      .addCase(createOrder.rejected, (state, action) => {
        state.loading.createOrder = false
        state.error = action.payload as string
      })
      .addCase(cancelOrder.fulfilled, (state, action) => {
        const orderId = action.payload
        const order = state.orders.find(o => o.id === orderId)
        if (order) {
          order.status = 'cancelled'
        }
      })
  },
})

export const { setSelectedPair, updateOrder, addTrade, clearError } = tradingSlice.actions

export const selectTrading = (state: { trading: TradingState }) => state.trading
export const selectOrders = (state: { trading: TradingState }) => state.trading.orders
export const selectTrades = (state: { trading: TradingState }) => state.trading.trades
export const selectTradingPairs = (state: { trading: TradingState }) => state.trading.tradingPairs
export const selectSelectedPair = (state: { trading: TradingState }) => state.trading.selectedPair

export default tradingSlice.reducer
