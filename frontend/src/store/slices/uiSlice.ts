/**
 * FlowEx UI State Redux Slice
 * 
 * Manages global UI state including theme, sidebar, notifications,
 * loading states, and user interface preferences.
 */

import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { UIState, Notification } from '../../types'

// Initial state - Default to dark mode with warm colors
const initialState: UIState = {
  theme: {
    mode: 'dark',
    primaryColor: '#3b82f6',
    fontSize: 'medium',
  },
  sidebar: {
    isOpen: true,
    isCollapsed: false,
  },
  notifications: [],
  loading: {},
}

// UI slice
const uiSlice = createSlice({
  name: 'ui',
  initialState,
  reducers: {
    // Theme actions
    setThemeMode: (state, action: PayloadAction<'light' | 'dark'>) => {
      state.theme.mode = action.payload
    },
    
    setPrimaryColor: (state, action: PayloadAction<string>) => {
      state.theme.primaryColor = action.payload
    },
    
    setFontSize: (state, action: PayloadAction<'small' | 'medium' | 'large'>) => {
      state.theme.fontSize = action.payload
    },
    
    toggleTheme: (state) => {
      state.theme.mode = state.theme.mode === 'light' ? 'dark' : 'light'
    },
    
    // Sidebar actions
    toggleSidebar: (state) => {
      state.sidebar.isOpen = !state.sidebar.isOpen
    },
    
    setSidebarOpen: (state, action: PayloadAction<boolean>) => {
      state.sidebar.isOpen = action.payload
    },
    
    toggleSidebarCollapsed: (state) => {
      state.sidebar.isCollapsed = !state.sidebar.isCollapsed
    },
    
    setSidebarCollapsed: (state, action: PayloadAction<boolean>) => {
      state.sidebar.isCollapsed = action.payload
    },
    
    // Notification actions
    addNotification: (state, action: PayloadAction<Omit<Notification, 'id' | 'timestamp' | 'read'>>) => {
      const notification: Notification = {
        ...action.payload,
        id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
        timestamp: new Date().toISOString(),
        read: false,
      }
      state.notifications.unshift(notification)
      
      // Keep only last 50 notifications
      if (state.notifications.length > 50) {
        state.notifications = state.notifications.slice(0, 50)
      }
    },
    
    removeNotification: (state, action: PayloadAction<string>) => {
      state.notifications = state.notifications.filter(
        notification => notification.id !== action.payload
      )
    },
    
    markNotificationAsRead: (state, action: PayloadAction<string>) => {
      const notification = state.notifications.find(n => n.id === action.payload)
      if (notification) {
        notification.read = true
      }
    },
    
    markAllNotificationsAsRead: (state) => {
      state.notifications.forEach(notification => {
        notification.read = true
      })
    },
    
    clearNotifications: (state) => {
      state.notifications = []
    },
    
    clearReadNotifications: (state) => {
      state.notifications = state.notifications.filter(n => !n.read)
    },
    
    // Loading actions
    setLoading: (state, action: PayloadAction<{ key: string; loading: boolean }>) => {
      const { key, loading } = action.payload
      if (loading) {
        state.loading[key] = true
      } else {
        delete state.loading[key]
      }
    },
    
    clearAllLoading: (state) => {
      state.loading = {}
    },
    
    // Reset UI state
    resetUI: () => initialState,
  },
})

// Export actions
export const {
  setThemeMode,
  setPrimaryColor,
  setFontSize,
  toggleTheme,
  toggleSidebar,
  setSidebarOpen,
  toggleSidebarCollapsed,
  setSidebarCollapsed,
  addNotification,
  removeNotification,
  markNotificationAsRead,
  markAllNotificationsAsRead,
  clearNotifications,
  clearReadNotifications,
  setLoading,
  clearAllLoading,
  resetUI,
} = uiSlice.actions

// Selectors
export const selectUI = (state: { ui: UIState }) => state.ui
export const selectTheme = (state: { ui: UIState }) => state.ui.theme
export const selectSidebar = (state: { ui: UIState }) => state.ui.sidebar
export const selectNotifications = (state: { ui: UIState }) => state.ui.notifications
export const selectLoading = (state: { ui: UIState }) => state.ui.loading

// Helper selectors
export const selectIsDarkMode = (state: { ui: UIState }) => 
  state.ui.theme.mode === 'dark'

export const selectUnreadNotifications = (state: { ui: UIState }) => 
  state.ui.notifications.filter(n => !n.read)

export const selectUnreadNotificationCount = (state: { ui: UIState }) => 
  state.ui.notifications.filter(n => !n.read).length

export const selectIsLoading = (key: string) => 
  (state: { ui: UIState }) => 
    Boolean(state.ui.loading[key])

export const selectAnyLoading = (state: { ui: UIState }) => 
  Object.keys(state.ui.loading).length > 0

export default uiSlice.reducer
