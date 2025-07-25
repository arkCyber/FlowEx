/**
 * FlowEx Login Page with Tailwind CSS
 */

import React, { useState } from 'react'
import { useDispatch } from 'react-redux'
import { Link, useNavigate } from 'react-router-dom'
import { Eye, EyeOff, LogIn, Mail, Lock } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'
import { loginUser } from '../../store/slices/authSlice'
import type { AppDispatch } from '../../store'
import { ButtonSpinner } from '../../components/LoadingSpinner'
import { cn } from '../../utils/cn'

// Validation schema
const schema = yup.object({
  email: yup
    .string()
    .email('Please enter a valid email')
    .required('Email is required'),
  password: yup
    .string()
    .min(6, 'Password must be at least 6 characters')
    .required('Password is required'),
  rememberMe: yup.boolean().optional(),
})

const Login: React.FC = () => {
  const dispatch = useDispatch<AppDispatch>()
  const navigate = useNavigate()
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)
  const [showPassword, setShowPassword] = useState(false)

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: yupResolver(schema),
    defaultValues: {
      email: '',
      password: '',
      rememberMe: false,
    },
  })

  const onSubmit = async (data: any) => {
    try {
      setLoading(true)
      setError(null)

      await dispatch(loginUser(data)).unwrap()
      navigate('/dashboard')
    } catch (err: any) {
      setError(err.message || 'Login failed')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="w-full">
      <div className="card p-8">
        {/* Logo and Title */}
        <div className="text-center mb-8">
          <div className="flex justify-center mb-4">
            <div className="w-12 h-12 bg-gradient-to-br from-primary-500 to-primary-700 rounded-xl flex items-center justify-center">
              <span className="text-white font-bold text-xl">F</span>
            </div>
          </div>

          <h1 className="text-3xl font-bold text-white mb-2">
            Welcome to FlowEx
          </h1>

          <p className="text-warm-400">
            Sign in to your trading account
          </p>
        </div>

        {/* Error Alert */}
        {error && (
          <div className="mb-6 p-4 bg-red-900/20 border border-red-500/30 rounded-lg">
            <p className="text-red-400 text-sm">{error}</p>
          </div>
        )}

        {/* Login Form */}
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Email Field */}
          <div>
            <label htmlFor="email" className="block text-sm font-medium text-warm-300 mb-2">
              Email Address
            </label>
            <div className="relative">
              <Mail size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-warm-500" />
              <input
                id="email"
                type="email"
                autoComplete="email"
                autoFocus
                className={cn(
                  'input pl-10 w-full',
                  errors.email && 'border-red-500 focus:ring-red-500'
                )}
                placeholder="Enter your email"
                {...register('email')}
              />
            </div>
            {errors.email && (
              <p className="mt-1 text-sm text-red-400">{errors.email.message}</p>
            )}
          </div>

          {/* Password Field */}
          <div>
            <label htmlFor="password" className="block text-sm font-medium text-warm-300 mb-2">
              Password
            </label>
            <div className="relative">
              <Lock size={18} className="absolute left-3 top-1/2 transform -translate-y-1/2 text-warm-500" />
              <input
                id="password"
                type={showPassword ? 'text' : 'password'}
                autoComplete="current-password"
                className={cn(
                  'input pl-10 pr-10 w-full',
                  errors.password && 'border-red-500 focus:ring-red-500'
                )}
                placeholder="Enter your password"
                {...register('password')}
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-1/2 transform -translate-y-1/2 text-warm-500 hover:text-warm-400"
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
            {errors.password && (
              <p className="mt-1 text-sm text-red-400">{errors.password.message}</p>
            )}
          </div>

          {/* Remember Me */}
          <div className="flex items-center">
            <input
              id="rememberMe"
              type="checkbox"
              className="w-4 h-4 text-primary-600 bg-background-warm-tertiary border-warm-600 rounded focus:ring-primary-500 focus:ring-2"
              {...register('rememberMe')}
            />
            <label htmlFor="rememberMe" className="ml-2 text-sm text-warm-300">
              Remember me
            </label>
          </div>

          {/* Submit Button */}
          <button
            type="submit"
            disabled={loading}
            className={cn(
              'w-full btn-primary flex items-center justify-center gap-2 py-3',
              loading && 'opacity-50 cursor-not-allowed'
            )}
          >
            {loading ? (
              <>
                <ButtonSpinner />
                Signing In...
              </>
            ) : (
              <>
                <LogIn size={18} />
                Sign In
              </>
            )}
          </button>

          {/* Links */}
          <div className="text-center space-y-2">
            <Link
              to="/auth/forgot-password"
              className="link text-sm hover:underline"
            >
              Forgot password?
            </Link>
            <div className="text-warm-400 text-sm">
              Don't have an account?{' '}
              <Link
                to="/auth/register"
                className="link hover:underline"
              >
                Sign Up
              </Link>
            </div>
          </div>
        </form>
      </div>
    </div>
  )
}

export default Login
