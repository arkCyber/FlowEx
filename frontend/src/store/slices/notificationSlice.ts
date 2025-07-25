/**
 * FlowEx Notification Redux Slice
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { Notification } from '../../types'

interface NotificationState {
  notifications: Notification[]
}

const initialState: NotificationState = {
  notifications: [],
}

const notificationSlice = createSlice({
  name: 'notifications',
  initialState,
  reducers: {
    addNotification: (state, action: PayloadAction<Omit<Notification, 'id' | 'timestamp' | 'read'>>) => {
      const notification: Notification = {
        ...action.payload,
        id: Date.now().toString(),
        timestamp: new Date().toISOString(),
        read: false,
      }
      state.notifications.unshift(notification)
    },
    removeNotification: (state, action: PayloadAction<string>) => {
      state.notifications = state.notifications.filter(n => n.id !== action.payload)
    },
  },
})

export const { addNotification, removeNotification } = notificationSlice.actions
export default notificationSlice.reducer
