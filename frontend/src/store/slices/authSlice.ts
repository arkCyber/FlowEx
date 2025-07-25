/**
 * FlowEx Authentication Redux Slice
 * 
 * Manages user authentication state, login/logout actions,
 * token management, and user permissions.
 */

import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit'
import { authApi } from '../../services/authApi'
import type {
  AuthState,
  User,
  LoginRequest,
  RegisterRequest
} from '../../types'

// Initial state
const initialState: AuthState = {
  isAuthenticated: false,
  user: null,
  token: null,
  refreshToken: null,
  loading: false,
  error: null,
}

// Async thunks
export const loginUser = createAsyncThunk(
  'auth/login',
  async (credentials: LoginRequest, { rejectWithValue }) => {
    try {
      const response = await authApi.login(credentials)
      
      // Store tokens in localStorage for persistence
      if (credentials.rememberMe) {
        localStorage.setItem('flowex_token', response.token)
        localStorage.setItem('flowex_refresh_token', response.refreshToken)
      } else {
        sessionStorage.setItem('flowex_token', response.token)
        sessionStorage.setItem('flowex_refresh_token', response.refreshToken)
      }
      
      return response
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Login failed'
      )
    }
  }
)

export const registerUser = createAsyncThunk(
  'auth/register',
  async (userData: RegisterRequest, { rejectWithValue }) => {
    try {
      const response = await authApi.register(userData)
      
      // Store tokens after successful registration
      localStorage.setItem('flowex_token', response.token)
      localStorage.setItem('flowex_refresh_token', response.refreshToken)
      
      return response
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Registration failed'
      )
    }
  }
)

export const refreshToken = createAsyncThunk(
  'auth/refreshToken',
  async (_, { getState, rejectWithValue }) => {
    try {
      const state = getState() as { auth: AuthState }
      const currentRefreshToken = state.auth.refreshToken
      
      if (!currentRefreshToken) {
        throw new Error('No refresh token available')
      }
      
      const response = await authApi.refreshToken(currentRefreshToken)
      
      // Update stored tokens
      const storage = localStorage.getItem('flowex_token') ? localStorage : sessionStorage
      storage.setItem('flowex_token', response.token)
      storage.setItem('flowex_refresh_token', response.refreshToken)
      
      return response
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Token refresh failed'
      )
    }
  }
)

export const getCurrentUser = createAsyncThunk(
  'auth/getCurrentUser',
  async (_, { rejectWithValue }) => {
    try {
      const user = await authApi.getCurrentUser()
      return user
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Failed to get user data'
      )
    }
  }
)

export const logoutUser = createAsyncThunk(
  'auth/logout',
  async (_, { getState }) => {
    try {
      const state = getState() as { auth: AuthState }
      if (state.auth.token) {
        await authApi.logout()
      }
    } catch (error) {
      // Continue with logout even if API call fails
      console.warn('Logout API call failed:', error)
    } finally {
      // Always clear local storage
      localStorage.removeItem('flowex_token')
      localStorage.removeItem('flowex_refresh_token')
      sessionStorage.removeItem('flowex_token')
      sessionStorage.removeItem('flowex_refresh_token')
    }
  }
)

export const updateProfile = createAsyncThunk(
  'auth/updateProfile',
  async (profileData: Partial<User>, { rejectWithValue }) => {
    try {
      const updatedUser = await authApi.updateProfile(profileData)
      return updatedUser
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Profile update failed'
      )
    }
  }
)

export const changePassword = createAsyncThunk(
  'auth/changePassword',
  async (
    passwordData: { currentPassword: string; newPassword: string },
    { rejectWithValue }
  ) => {
    try {
      await authApi.changePassword(passwordData)
      return { success: true }
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || 'Password change failed'
      )
    }
  }
)

export const enable2FA = createAsyncThunk(
  'auth/enable2FA',
  async (code: string, { rejectWithValue }) => {
    try {
      const result = await authApi.enable2FA(code)
      return result
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || '2FA setup failed'
      )
    }
  }
)

export const disable2FA = createAsyncThunk(
  'auth/disable2FA',
  async (code: string, { rejectWithValue }) => {
    try {
      await authApi.disable2FA(code)
      return { twoFactorEnabled: false }
    } catch (error: any) {
      return rejectWithValue(
        error.response?.data?.message || '2FA disable failed'
      )
    }
  }
)

// Auth slice
const authSlice = createSlice({
  name: 'auth',
  initialState,
  reducers: {
    // Clear error
    clearError: (state) => {
      state.error = null
    },
    
    // Set loading state
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload
    },
    
    // Initialize auth from stored tokens
    initializeAuth: (state, action: PayloadAction<{ token: string; refreshToken: string }>) => {
      state.token = action.payload.token
      state.refreshToken = action.payload.refreshToken
      state.isAuthenticated = true
    },
    
    // Update user data
    updateUser: (state, action: PayloadAction<Partial<User>>) => {
      if (state.user) {
        state.user = { ...state.user, ...action.payload }
      }
    },
    
    // Reset auth state
    resetAuth: () => initialState,
  },
  extraReducers: (builder) => {
    // Login
    builder
      .addCase(loginUser.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(loginUser.fulfilled, (state, action) => {
        state.loading = false
        state.isAuthenticated = true
        state.user = action.payload.user
        state.token = action.payload.token
        state.refreshToken = action.payload.refreshToken
        state.error = null
      })
      .addCase(loginUser.rejected, (state, action) => {
        state.loading = false
        state.isAuthenticated = false
        state.user = null
        state.token = null
        state.refreshToken = null
        state.error = action.payload as string
      })
    
    // Register
    builder
      .addCase(registerUser.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(registerUser.fulfilled, (state, action) => {
        state.loading = false
        state.isAuthenticated = true
        state.user = action.payload.user
        state.token = action.payload.token
        state.refreshToken = action.payload.refreshToken
        state.error = null
      })
      .addCase(registerUser.rejected, (state, action) => {
        state.loading = false
        state.error = action.payload as string
      })
    
    // Refresh token
    builder
      .addCase(refreshToken.fulfilled, (state, action) => {
        state.token = action.payload.token
        state.refreshToken = action.payload.refreshToken
        state.user = action.payload.user
      })
      .addCase(refreshToken.rejected, (state) => {
        state.isAuthenticated = false
        state.user = null
        state.token = null
        state.refreshToken = null
      })
    
    // Get current user
    builder
      .addCase(getCurrentUser.fulfilled, (state, action) => {
        state.user = action.payload
        state.isAuthenticated = true
      })
      .addCase(getCurrentUser.rejected, (state) => {
        state.isAuthenticated = false
        state.user = null
        state.token = null
        state.refreshToken = null
      })
    
    // Logout
    builder.addCase(logoutUser.fulfilled, (state) => {
      state.isAuthenticated = false
      state.user = null
      state.token = null
      state.refreshToken = null
      state.error = null
    })
    
    // Update profile
    builder
      .addCase(updateProfile.fulfilled, (state, action) => {
        state.user = action.payload
      })
      .addCase(updateProfile.rejected, (state, action) => {
        state.error = action.payload as string
      })
    
    // 2FA
    builder
      .addCase(enable2FA.fulfilled, (state) => {
        if (state.user) {
          state.user.twoFactorEnabled = true
        }
      })
      .addCase(disable2FA.fulfilled, (state) => {
        if (state.user) {
          state.user.twoFactorEnabled = false
        }
      })
  },
})

// Export actions
export const {
  clearError,
  setLoading,
  initializeAuth,
  updateUser,
  resetAuth,
} = authSlice.actions

// Selectors
export const selectAuth = (state: { auth: AuthState }) => state.auth
export const selectUser = (state: { auth: AuthState }) => state.auth.user
export const selectIsAuthenticated = (state: { auth: AuthState }) => state.auth.isAuthenticated
export const selectAuthLoading = (state: { auth: AuthState }) => state.auth.loading
export const selectAuthError = (state: { auth: AuthState }) => state.auth.error

// Helper selectors
export const selectUserPermissions = (state: { auth: AuthState }) => 
  state.auth.user?.permissions || []

export const selectUserRoles = (state: { auth: AuthState }) => 
  state.auth.user?.roles || []

export const selectHasPermission = (permission: string) => 
  (state: { auth: AuthState }) => 
    state.auth.user?.permissions.includes(permission) || false

export const selectHasRole = (role: string) => 
  (state: { auth: AuthState }) => 
    state.auth.user?.roles.includes(role) || false

export const selectHasAnyPermission = (permissions: string[]) => 
  (state: { auth: AuthState }) => 
    permissions.some(permission => 
      state.auth.user?.permissions.includes(permission)
    )

export const selectHasAllPermissions = (permissions: string[]) => 
  (state: { auth: AuthState }) => 
    permissions.every(permission => 
      state.auth.user?.permissions.includes(permission)
    )

export default authSlice.reducer
