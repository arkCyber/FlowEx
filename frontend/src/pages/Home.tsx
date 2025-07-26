/**
 * FlowEx Home Page
 * 
 * Landing page showcasing the professional trading platform
 * Features hero section, market overview, and quick access to trading
 */

import React from 'react'
import { Link } from 'react-router-dom'
import { useTheme } from '../hooks/useTheme'
import { 
  ChartBarIcon, 
  CurrencyDollarIcon, 
  ShieldCheckIcon,
  RocketLaunchIcon,
  ArrowRightIcon
} from '@heroicons/react/24/outline'

const Home: React.FC = () => {
  const { isDarkMode } = useTheme()
  
  const features = [
    {
      icon: ChartBarIcon,
      title: 'Advanced Trading',
      description: 'Professional trading tools with real-time charts and technical indicators'
    },
    {
      icon: ShieldCheckIcon,
      title: 'Bank-Level Security',
      description: 'Multi-layer security with encryption and compliance standards'
    },
    {
      icon: CurrencyDollarIcon,
      title: 'Low Fees',
      description: 'Competitive trading fees with volume-based discounts'
    },
    {
      icon: RocketLaunchIcon,
      title: 'High Performance',
      description: 'Sub-100ms order processing with 99.9% uptime guarantee'
    }
  ]
  
  const mockMarketData = [
    { symbol: 'BTC/USDT', price: '45,234.56', change: '+2.81%', isPositive: true },
    { symbol: 'ETH/USDT', price: '2,834.12', change: '+1.45%', isPositive: true },
    { symbol: 'ADA/USDT', price: '0.4567', change: '-0.89%', isPositive: false },
    { symbol: 'DOT/USDT', price: '12.34', change: '+3.21%', isPositive: true }
  ]
  
  return (
    <div className={`min-h-screen ${
      isDarkMode ? 'bg-dark-950' : 'bg-gray-50'
    }`}>
      {/* Hero Section */}
      <div className={`relative overflow-hidden ${
        isDarkMode ? 'bg-dark-900' : 'bg-white'
      }`}>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
          <div className="text-center">
            <h1 className={`text-4xl md:text-6xl font-bold mb-6 ${
              isDarkMode ? 'text-dark-50' : 'text-gray-900'
            }`}>
              Welcome to{' '}
              <span className="text-primary-500">FlowEx</span>
            </h1>
            <p className={`text-xl md:text-2xl mb-8 max-w-3xl mx-auto ${
              isDarkMode ? 'text-dark-300' : 'text-gray-600'
            }`}>
              Next-generation enterprise trading platform with professional tools, 
              bank-level security, and lightning-fast execution
            </p>
            
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link
                to="/trading"
                className="inline-flex items-center px-8 py-4 bg-primary-500 hover:bg-primary-600 text-white font-semibold rounded-lg transition-colors"
              >
                Start Trading
                <ArrowRightIcon className="ml-2 w-5 h-5" />
              </Link>
              
              <Link
                to="/markets"
                className={`inline-flex items-center px-8 py-4 border-2 font-semibold rounded-lg transition-colors ${
                  isDarkMode
                    ? 'border-dark-700 text-dark-200 hover:bg-dark-800'
                    : 'border-gray-300 text-gray-700 hover:bg-gray-50'
                }`}
              >
                View Markets
              </Link>
            </div>
          </div>
        </div>
        
        {/* Background decoration */}
        <div className="absolute inset-0 -z-10">
          <div className="absolute top-0 left-1/2 transform -translate-x-1/2 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl"></div>
          <div className="absolute bottom-0 right-1/4 w-64 h-64 bg-success-500/10 rounded-full blur-3xl"></div>
        </div>
      </div>
      
      {/* Market Overview */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className="text-center mb-12">
          <h2 className={`text-3xl font-bold mb-4 ${
            isDarkMode ? 'text-dark-50' : 'text-gray-900'
          }`}>
            Live Market Data
          </h2>
          <p className={`text-lg ${
            isDarkMode ? 'text-dark-400' : 'text-gray-600'
          }`}>
            Real-time cryptocurrency prices and market movements
          </p>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
          {mockMarketData.map((market, index) => (
            <div
              key={index}
              className={`p-6 rounded-xl border transition-colors ${
                isDarkMode
                  ? 'bg-dark-900 border-dark-800 hover:bg-dark-800'
                  : 'bg-white border-gray-200 hover:bg-gray-50'
              }`}
            >
              <div className="flex items-center justify-between mb-2">
                <span className={`font-semibold ${
                  isDarkMode ? 'text-dark-200' : 'text-gray-700'
                }`}>
                  {market.symbol}
                </span>
                <span className={`text-sm px-2 py-1 rounded ${
                  market.isPositive
                    ? 'bg-success-500/10 text-success-500'
                    : 'bg-danger-500/10 text-danger-500'
                }`}>
                  {market.change}
                </span>
              </div>
              <div className={`text-2xl font-bold font-mono ${
                isDarkMode ? 'text-dark-50' : 'text-gray-900'
              }`}>
                ${market.price}
              </div>
            </div>
          ))}
        </div>
      </div>
      
      {/* Features Section */}
      <div className={`py-16 ${
        isDarkMode ? 'bg-dark-900' : 'bg-white'
      }`}>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className={`text-3xl font-bold mb-4 ${
              isDarkMode ? 'text-dark-50' : 'text-gray-900'
            }`}>
              Why Choose FlowEx?
            </h2>
            <p className={`text-lg ${
              isDarkMode ? 'text-dark-400' : 'text-gray-600'
            }`}>
              Professional trading platform built for serious traders
            </p>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            {features.map((feature, index) => {
              const Icon = feature.icon
              return (
                <div key={index} className="text-center">
                  <div className={`w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center ${
                    isDarkMode ? 'bg-dark-800' : 'bg-gray-100'
                  }`}>
                    <Icon className="w-8 h-8 text-primary-500" />
                  </div>
                  <h3 className={`text-xl font-semibold mb-2 ${
                    isDarkMode ? 'text-dark-50' : 'text-gray-900'
                  }`}>
                    {feature.title}
                  </h3>
                  <p className={`${
                    isDarkMode ? 'text-dark-400' : 'text-gray-600'
                  }`}>
                    {feature.description}
                  </p>
                </div>
              )
            })}
          </div>
        </div>
      </div>
      
      {/* CTA Section */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        <div className={`text-center p-12 rounded-2xl ${
          isDarkMode ? 'bg-dark-900' : 'bg-white'
        }`}>
          <h2 className={`text-3xl font-bold mb-4 ${
            isDarkMode ? 'text-dark-50' : 'text-gray-900'
          }`}>
            Ready to Start Trading?
          </h2>
          <p className={`text-lg mb-8 ${
            isDarkMode ? 'text-dark-400' : 'text-gray-600'
          }`}>
            Join thousands of traders using FlowEx for professional cryptocurrency trading
          </p>
          
          <Link
            to="/trading"
            className="inline-flex items-center px-8 py-4 bg-primary-500 hover:bg-primary-600 text-white font-semibold rounded-lg transition-colors"
          >
            Launch Trading Platform
            <RocketLaunchIcon className="ml-2 w-5 h-5" />
          </Link>
        </div>
      </div>
    </div>
  )
}

export default Home
