/**
 * FlowEx Error Handling Middleware
 */

import { Middleware } from '@reduxjs/toolkit'
import { addNotification } from '../slices/uiSlice'

export const rtkQueryErrorLogger: Middleware = (api) => (next) => (action) => {
  if (action.type.endsWith('/rejected')) {
    const error = action.payload || action.error
    
    api.dispatch(addNotification({
      type: 'error',
      title: 'Error',
      message: error.message || 'An error occurred',
    }))
  }
  
  return next(action)
}
