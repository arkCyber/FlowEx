/**
 * FlowEx API Client
 * 
 * Enterprise-grade API client with authentication, error handling,
 * request/response interceptors, and comprehensive logging.
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { store } from '../store'
import { refreshToken, logoutUser } from '../store/slices/authSlice'
import { addNotification } from '../store/slices/uiSlice'

// API configuration
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8001'
const API_TIMEOUT = 30000 // 30 seconds

// Create axios instance
const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: API_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },
})

// Request interceptor
apiClient.interceptors.request.use(
  (config) => {
    // Add auth token
    const state = store.getState()
    const token = state.auth.token
    
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    
    // Add request ID for tracing
    config.headers['X-Request-ID'] = generateRequestId()
    
    // Log request in development
    if (import.meta.env.DEV) {
      console.log(`🚀 API Request: ${config.method?.toUpperCase()} ${config.url}`, {
        headers: config.headers,
        data: config.data,
      })
    }
    
    return config
  },
  (error) => {
    console.error('Request interceptor error:', error)
    return Promise.reject(error)
  }
)

// Response interceptor
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    // Log response in development
    if (import.meta.env.DEV) {
      console.log(`✅ API Response: ${response.config.method?.toUpperCase()} ${response.config.url}`, {
        status: response.status,
        data: response.data,
      })
    }
    
    return response
  },
  async (error) => {
    const originalRequest = error.config
    
    // Log error in development
    if (import.meta.env.DEV) {
      console.error(`❌ API Error: ${originalRequest?.method?.toUpperCase()} ${originalRequest?.url}`, {
        status: error.response?.status,
        data: error.response?.data,
        message: error.message,
      })
    }
    
    // Handle 401 Unauthorized
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true
      
      try {
        // Try to refresh token
        await store.dispatch(refreshToken()).unwrap()
        
        // Retry original request with new token
        const state = store.getState()
        const newToken = state.auth.token
        
        if (newToken) {
          originalRequest.headers.Authorization = `Bearer ${newToken}`
          return apiClient(originalRequest)
        }
      } catch (refreshError) {
        // Refresh failed, logout user
        store.dispatch(logoutUser())
        
        // Redirect to login
        if (window.location.pathname !== '/auth/login') {
          window.location.href = '/auth/login'
        }
        
        return Promise.reject(refreshError)
      }
    }
    
    // Handle network errors
    if (!error.response) {
      store.dispatch(addNotification({
        type: 'error',
        title: 'Network Error',
        message: 'Unable to connect to the server. Please check your internet connection.',
      }))
    }
    
    // Handle server errors
    if (error.response?.status >= 500) {
      store.dispatch(addNotification({
        type: 'error',
        title: 'Server Error',
        message: 'A server error occurred. Please try again later.',
      }))
    }
    
    // Handle rate limiting
    if (error.response?.status === 429) {
      store.dispatch(addNotification({
        type: 'warning',
        title: 'Rate Limited',
        message: 'Too many requests. Please wait a moment before trying again.',
      }))
    }
    
    return Promise.reject(error)
  }
)

// Utility functions
function generateRequestId(): string {
  return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
}

// API methods
export const api = {
  // Generic methods
  get: <T = any>(url: string, config?: AxiosRequestConfig): Promise<T> =>
    apiClient.get(url, config).then(response => response.data),
    
  post: <T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> =>
    apiClient.post(url, data, config).then(response => response.data),
    
  put: <T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> =>
    apiClient.put(url, data, config).then(response => response.data),
    
  patch: <T = any>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> =>
    apiClient.patch(url, data, config).then(response => response.data),
    
  delete: <T = any>(url: string, config?: AxiosRequestConfig): Promise<T> =>
    apiClient.delete(url, config).then(response => response.data),
    
  // File upload
  upload: <T = any>(url: string, file: File, onProgress?: (progress: number) => void): Promise<T> => {
    const formData = new FormData()
    formData.append('file', file)
    
    return apiClient.post(url, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total)
          onProgress(progress)
        }
      },
    }).then(response => response.data)
  },
  
  // Health check
  healthCheck: (): Promise<{ status: string; timestamp: string }> =>
    apiClient.get('/health').then(response => response.data),
}

// Export axios instance for advanced usage
export { apiClient }

export default api
