/**
 * FlowEx Market Data Redux Slice
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { Ticker, OrderBook } from '../../types'

interface MarketDataState {
  tickers: Record<string, Ticker>
  orderBooks: Record<string, OrderBook>
  connected: boolean
}

const initialState: MarketDataState = {
  tickers: {},
  orderBooks: {},
  connected: false,
}

const marketDataSlice = createSlice({
  name: 'marketData',
  initialState,
  reducers: {
    updateTicker: (state, action: PayloadAction<Ticker>) => {
      state.tickers[action.payload.symbol] = action.payload
    },
    updateOrderBook: (state, action: PayloadAction<OrderBook>) => {
      state.orderBooks[action.payload.symbol] = action.payload
    },
    setConnected: (state, action: PayloadAction<boolean>) => {
      state.connected = action.payload
    },
  },
})

export const { updateTicker, updateOrderBook, setConnected } = marketDataSlice.actions
export default marketDataSlice.reducer
