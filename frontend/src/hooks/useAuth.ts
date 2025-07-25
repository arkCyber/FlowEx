/**
 * FlowEx Authentication Hook
 */

import { useCallback } from 'react'
import { useSelector, useDispatch } from 'react-redux'
import type { AppDispatch } from '../store'
import {
  loginUser,
  logoutUser,
  getCurrentUser,
  initializeAuth,
  selectAuth,
  selectUser,
  selectIsAuthenticated,
  selectUserPermissions,
  selectUserRoles,
} from '../store/slices/authSlice'

export const useAuth = () => {
  const dispatch = useDispatch<AppDispatch>()
  const auth = useSelector(selectAuth)
  const user = useSelector(selectUser)
  const isAuthenticated = useSelector(selectIsAuthenticated)
  const permissions = useSelector(selectUserPermissions)
  const roles = useSelector(selectUserRoles)

  const login = useCallback(
    (credentials: { email: string; password: string; rememberMe?: boolean }) => {
      return dispatch(loginUser(credentials))
    },
    [dispatch]
  )

  const logout = useCallback(() => {
    return dispatch(logoutUser())
  }, [dispatch])

  const refreshUser = useCallback(() => {
    return dispatch(getCurrentUser())
  }, [dispatch])

  const initAuth = useCallback(() => {
    const token = localStorage.getItem('flowex_token') || sessionStorage.getItem('flowex_token')
    const refreshToken = localStorage.getItem('flowex_refresh_token') || sessionStorage.getItem('flowex_refresh_token')
    
    if (token && refreshToken) {
      dispatch(initializeAuth({ token, refreshToken }))
      dispatch(getCurrentUser())
    }
  }, [dispatch])

  const hasPermission = useCallback(
    (permission: string) => {
      return permissions.includes(permission)
    },
    [permissions]
  )

  const hasPermissions = useCallback(
    (requiredPermissions: string[]) => {
      return requiredPermissions.every(permission => permissions.includes(permission))
    },
    [permissions]
  )

  const hasRole = useCallback(
    (role: string) => {
      return roles.includes(role)
    },
    [roles]
  )

  return {
    ...auth,
    user,
    isAuthenticated,
    permissions,
    roles,
    login,
    logout,
    refreshUser,
    initializeAuth: initAuth,
    hasPermission,
    hasPermissions,
    hasRole,
  }
}
