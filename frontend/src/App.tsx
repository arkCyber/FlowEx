/**
 * FlowEx Main Application Component
 * 
 * Enterprise-grade React application with routing, authentication,
 * real-time data, and comprehensive error handling.
 */

import React, { Suspense, useEffect } from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'

// Layout components
import { MainLayout } from './components/Layout/MainLayout'
import { AuthLayout } from './components/Layout/AuthLayout'
import { LoadingSpinner } from './components/LoadingSpinner'

// Hooks
import { useAuth } from './hooks/useAuth'
import { useWebSocket } from './hooks/useWebSocket'
import { useTheme } from './hooks/useTheme'

// Types are imported in hooks

// Lazy load pages for code splitting
const Dashboard = React.lazy(() => import('./pages/Dashboard'))
const Trading = React.lazy(() => import('./pages/Trading'))
const Portfolio = React.lazy(() => import('./pages/Portfolio'))
const Markets = React.lazy(() => import('./pages/Markets'))
const Orders = React.lazy(() => import('./pages/Orders'))
const Wallet = React.lazy(() => import('./pages/Wallet'))
const Settings = React.lazy(() => import('./pages/Settings'))
const Profile = React.lazy(() => import('./pages/Profile'))
const Login = React.lazy(() => import('./pages/Auth/Login'))
const Register = React.lazy(() => import('./pages/Auth/Register'))
const ForgotPassword = React.lazy(() => import('./pages/Auth/ForgotPassword'))
const NotFound = React.lazy(() => import('./pages/NotFound'))

// Protected Route Component
interface ProtectedRouteProps {
  children: React.ReactNode
  requiredPermissions?: string[]
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ 
  children, 
  requiredPermissions = [] 
}) => {
  const { isAuthenticated, hasPermissions } = useAuth()
  
  if (!isAuthenticated) {
    return <Navigate to="/auth/login" replace />
  }
  
  if (requiredPermissions.length > 0 && !hasPermissions(requiredPermissions)) {
    return <Navigate to="/unauthorized" replace />
  }
  
  return <>{children}</>
}

// Public Route Component (redirect if authenticated)
interface PublicRouteProps {
  children: React.ReactNode
}

const PublicRoute: React.FC<PublicRouteProps> = ({ children }) => {
  const { isAuthenticated } = useAuth()
  
  if (isAuthenticated) {
    return <Navigate to="/dashboard" replace />
  }
  
  return <>{children}</>
}

// Main App Component
const App: React.FC = () => {
  const { isAuthenticated, initializeAuth } = useAuth()
  const { connectWebSocket, disconnectWebSocket } = useWebSocket()
  const { isDarkMode } = useTheme()
  
  // Initialize authentication on app start
  useEffect(() => {
    initializeAuth()
  }, [initializeAuth])
  
  // Connect WebSocket when authenticated
  useEffect(() => {
    if (isAuthenticated) {
      connectWebSocket()
    } else {
      disconnectWebSocket()
    }

    return () => {
      disconnectWebSocket()
    }
  }, [isAuthenticated, connectWebSocket, disconnectWebSocket])
  
  // Apply theme class to body
  useEffect(() => {
    document.body.className = isDarkMode ? 'dark-theme' : 'light-theme'
  }, [isDarkMode])
  
  return (
    <div className="min-h-screen bg-stone-900">
      <Suspense fallback={<LoadingSpinner fullScreen />}>
          <Routes>
            {/* Public Routes */}
            <Route path="/auth/*" element={
              <PublicRoute>
                <AuthLayout>
                  <Routes>
                    <Route path="login" element={<Login />} />
                    <Route path="register" element={<Register />} />
                    <Route path="forgot-password" element={<ForgotPassword />} />
                    <Route path="*" element={<Navigate to="/auth/login" replace />} />
                  </Routes>
                </AuthLayout>
              </PublicRoute>
            } />
            
            {/* Protected Routes */}
            <Route path="/*" element={
              <ProtectedRoute>
                <MainLayout>
                  <Routes>
                    {/* Dashboard */}
                    <Route path="/" element={<Navigate to="/dashboard" replace />} />
                    <Route path="/dashboard" element={<Dashboard />} />
                    
                    {/* Trading */}
                    <Route 
                      path="/trading" 
                      element={
                        <ProtectedRoute requiredPermissions={['trading:read']}>
                          <Trading />
                        </ProtectedRoute>
                      } 
                    />
                    
                    {/* Portfolio */}
                    <Route path="/portfolio" element={<Portfolio />} />
                    
                    {/* Markets */}
                    <Route path="/markets" element={<Markets />} />
                    
                    {/* Orders */}
                    <Route 
                      path="/orders" 
                      element={
                        <ProtectedRoute requiredPermissions={['trading:read']}>
                          <Orders />
                        </ProtectedRoute>
                      } 
                    />
                    
                    {/* Wallet */}
                    <Route 
                      path="/wallet" 
                      element={
                        <ProtectedRoute requiredPermissions={['wallet:read']}>
                          <Wallet />
                        </ProtectedRoute>
                      } 
                    />
                    
                    {/* Settings */}
                    <Route path="/settings" element={<Settings />} />
                    
                    {/* Profile */}
                    <Route path="/profile" element={<Profile />} />
                    
                    {/* 404 */}
                    <Route path="*" element={<NotFound />} />
                  </Routes>
                </MainLayout>
              </ProtectedRoute>
            } />
          </Routes>
        </Suspense>
      </div>
  )
}

export default App
