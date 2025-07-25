/**
 * FlowEx WebSocket Hook
 */

import { useCallback, useEffect, useRef } from 'react'
import { useDispatch } from 'react-redux'
import { io, Socket } from 'socket.io-client'
import type { AppDispatch } from '../store'
import { updateTicker, updateOrderBook, setConnected } from '../store/slices/marketDataSlice'
import { updateOrder, addTrade } from '../store/slices/tradingSlice'

export const useWebSocket = () => {
  const dispatch = useDispatch<AppDispatch>()
  const socketRef = useRef<Socket | null>(null)

  const connectWebSocket = useCallback(() => {
    if (socketRef.current?.connected) {
      return
    }

    const wsUrl = import.meta.env.VITE_WS_BASE_URL || 'ws://localhost:8001'
    const token = localStorage.getItem('flowex_token') || sessionStorage.getItem('flowex_token')

    socketRef.current = io(wsUrl, {
      auth: {
        token,
      },
      transports: ['websocket'],
    })

    const socket = socketRef.current

    socket.on('connect', () => {
      console.log('WebSocket connected')
      dispatch(setConnected(true))
    })

    socket.on('disconnect', () => {
      console.log('WebSocket disconnected')
      dispatch(setConnected(false))
    })

    socket.on('ticker_update', (data) => {
      dispatch(updateTicker(data))
    })

    socket.on('orderbook_update', (data) => {
      dispatch(updateOrderBook(data))
    })

    socket.on('order_update', (data) => {
      dispatch(updateOrder(data))
    })

    socket.on('trade_update', (data) => {
      dispatch(addTrade(data))
    })

    socket.on('error', (error) => {
      console.error('WebSocket error:', error)
    })
  }, [dispatch])

  const disconnectWebSocket = useCallback(() => {
    if (socketRef.current) {
      socketRef.current.disconnect()
      socketRef.current = null
      dispatch(setConnected(false))
    }
  }, [dispatch])

  const subscribeToSymbol = useCallback((symbol: string) => {
    if (socketRef.current?.connected) {
      socketRef.current.emit('subscribe', { symbol })
    }
  }, [])

  const unsubscribeFromSymbol = useCallback((symbol: string) => {
    if (socketRef.current?.connected) {
      socketRef.current.emit('unsubscribe', { symbol })
    }
  }, [])

  useEffect(() => {
    return () => {
      disconnectWebSocket()
    }
  }, [disconnectWebSocket])

  return {
    connectWebSocket,
    disconnectWebSocket,
    subscribeToSymbol,
    unsubscribeFromSymbol,
  }
}
