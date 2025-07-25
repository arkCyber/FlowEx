/**
 * FlowEx Theme Hook with Tailwind CSS
 */

import { useSelector, useDispatch } from 'react-redux'
import { useCallback, useEffect } from 'react'
import type { AppDispatch } from '../store'
import {
  setThemeMode,
  selectTheme,
  selectIsDarkMode,
} from '../store/slices/uiSlice'

export const useTheme = () => {
  const dispatch = useDispatch<AppDispatch>()
  const theme = useSelector(selectTheme)
  const isDarkMode = useSelector(selectIsDarkMode)

  // Apply theme class to document
  useEffect(() => {
    const root = document.documentElement

    if (isDarkMode) {
      root.classList.add('dark')
      root.classList.remove('light')
    } else {
      root.classList.add('light')
      root.classList.remove('dark')
    }
  }, [isDarkMode])

  // Initialize theme on mount
  useEffect(() => {
    const savedTheme = localStorage.getItem('flowex-theme')

    if (savedTheme) {
      dispatch(setThemeMode(savedTheme as 'light' | 'dark'))
    } else {
      // Default to dark mode as requested
      dispatch(setThemeMode('dark'))
    }
  }, [dispatch])

  const setMode = useCallback(
    (mode: 'light' | 'dark') => {
      dispatch(setThemeMode(mode))
      localStorage.setItem('flowex-theme', mode)
    },
    [dispatch]
  )

  const toggle = useCallback(() => {
    const newMode = isDarkMode ? 'light' : 'dark'
    setMode(newMode)
  }, [isDarkMode, setMode])

  return {
    theme,
    isDarkMode,
    setMode,
    toggle,
  }
}
