/**
 * FlowEx Wallet Redux Slice
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { Balance, Transaction } from '../../types'

interface WalletState {
  balances: Balance[]
  transactions: Transaction[]
  loading: boolean
}

const initialState: WalletState = {
  balances: [],
  transactions: [],
  loading: false,
}

const walletSlice = createSlice({
  name: 'wallet',
  initialState,
  reducers: {
    setBalances: (state, action: PayloadAction<Balance[]>) => {
      state.balances = action.payload
    },
    setTransactions: (state, action: PayloadAction<Transaction[]>) => {
      state.transactions = action.payload
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload
    },
  },
})

export const { setBalances, setTransactions, setLoading } = walletSlice.actions
export default walletSlice.reducer
