/**
 * FlowEx Authentication API Service
 */

import api from './api'
import type { 
  LoginRequest, 
  RegisterRequest, 
  LoginResponse, 
  User 
} from '../types'

export const authApi = {
  // Authentication
  login: (credentials: LoginRequest): Promise<LoginResponse> =>
    api.post('/api/auth/login', credentials),
    
  register: (userData: RegisterRequest): Promise<LoginResponse> =>
    api.post('/api/auth/register', userData),
    
  logout: (): Promise<void> =>
    api.post('/api/auth/logout'),
    
  refreshToken: (refreshToken: string): Promise<LoginResponse> =>
    api.post('/api/auth/refresh', { refreshToken }),
    
  // User management
  getCurrentUser: (): Promise<User> =>
    api.get('/api/auth/me'),
    
  updateProfile: (profileData: Partial<User>): Promise<User> =>
    api.patch('/api/auth/profile', profileData),
    
  changePassword: (passwordData: { currentPassword: string; newPassword: string }): Promise<void> =>
    api.post('/api/auth/change-password', passwordData),
    
  // Two-factor authentication
  enable2FA: (code: string): Promise<{ secret: string; qrCode: string }> =>
    api.post('/api/auth/2fa/enable', { code }),
    
  disable2FA: (code: string): Promise<void> =>
    api.post('/api/auth/2fa/disable', { code }),
    
  verify2FA: (code: string): Promise<void> =>
    api.post('/api/auth/2fa/verify', { code }),
    
  // Password reset
  forgotPassword: (email: string): Promise<void> =>
    api.post('/api/auth/forgot-password', { email }),
    
  resetPassword: (token: string, newPassword: string): Promise<void> =>
    api.post('/api/auth/reset-password', { token, newPassword }),
    
  // Email verification
  verifyEmail: (token: string): Promise<void> =>
    api.post('/api/auth/verify-email', { token }),
    
  resendVerification: (): Promise<void> =>
    api.post('/api/auth/resend-verification'),
}
