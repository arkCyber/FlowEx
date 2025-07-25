/**
 * LoadingSpinner Component Tests
 */

import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { LoadingSpinner } from '../LoadingSpinner'

describe('LoadingSpinner', () => {
  it('renders with default message', () => {
    render(<LoadingSpinner />)
    expect(screen.getByText('Loading...')).toBeInTheDocument()
  })

  it('renders with custom message', () => {
    render(<LoadingSpinner message="Custom loading message" />)
    expect(screen.getByText('Custom loading message')).toBeInTheDocument()
  })

  it('renders without message when not provided', () => {
    render(<LoadingSpinner message="" />)
    expect(screen.queryByText('Loading...')).not.toBeInTheDocument()
  })
})
