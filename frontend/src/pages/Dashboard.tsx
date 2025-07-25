/**
 * FlowEx Dashboard Page with Tailwind CSS
 */

import React from 'react'
import {
  TrendingUp,
  Wallet,
  ShoppingCart,
  BarChart3,
  DollarSign,
  Activity,
  Users
} from 'lucide-react'
import { useAuth } from '../hooks/useAuth'
import { cn, formatPercentageChange } from '../utils/cn'

const Dashboard: React.FC = () => {
  const { user } = useAuth()

  // Mock data - replace with real data from API
  const portfolioData = {
    totalValue: 125420.50,
    dayChange: 2.34,
    dayChangeAmount: 2890.45,
    openOrders: 3,
    totalTrades: 127,
    availableBalance: 45230.20,
    totalProfit: 15420.30,
    winRate: 68.5,
  }

  const stats = [
    {
      title: 'Portfolio Value',
      value: `$${portfolioData.totalValue.toLocaleString()}`,
      change: portfolioData.dayChange,
      changeAmount: portfolioData.dayChangeAmount,
      icon: Wallet,
      color: 'text-primary-400',
      bgColor: 'bg-primary-500/10',
    },
    {
      title: '24h P&L',
      value: `$${portfolioData.dayChangeAmount.toLocaleString()}`,
      change: portfolioData.dayChange,
      icon: TrendingUp,
      color: 'text-trading-buy',
      bgColor: 'bg-green-500/10',
    },
    {
      title: 'Open Orders',
      value: portfolioData.openOrders.toString(),
      icon: ShoppingCart,
      color: 'text-warm-400',
      bgColor: 'bg-warm-500/10',
    },
    {
      title: 'Total Trades',
      value: portfolioData.totalTrades.toString(),
      icon: BarChart3,
      color: 'text-blue-400',
      bgColor: 'bg-blue-500/10',
    },
  ]

  const additionalStats = [
    {
      title: 'Available Balance',
      value: `$${portfolioData.availableBalance.toLocaleString()}`,
      icon: DollarSign,
    },
    {
      title: 'Total Profit',
      value: `$${portfolioData.totalProfit.toLocaleString()}`,
      icon: Activity,
    },
    {
      title: 'Win Rate',
      value: `${portfolioData.winRate}%`,
      icon: Users,
    },
  ]

  return (
    <div className="space-y-8">
      {/* Welcome Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">
            Welcome back, {user?.firstName || 'Trader'}!
          </h1>
          <p className="text-warm-400 mt-1">
            Here's what's happening with your portfolio today.
          </p>
        </div>
        <div className="text-right">
          <div className="text-sm text-warm-400">
            {new Date().toLocaleDateString('en-US', {
              weekday: 'long',
              year: 'numeric',
              month: 'long',
              day: 'numeric',
            })}
          </div>
        </div>
      </div>

      {/* Main Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {stats.map((stat, index) => {
          const changeData = stat.change ? formatPercentageChange(stat.change) : null

          return (
            <div key={index} className="card hover:shadow-warm-lg transition-shadow duration-200">
              <div className="flex items-center justify-between mb-4">
                <div className={cn('p-3 rounded-lg', stat.bgColor)}>
                  <stat.icon size={24} className={stat.color} />
                </div>
                {changeData && (
                  <div className={changeData.className}>
                    {changeData.text}
                  </div>
                )}
              </div>

              <div>
                <h3 className="text-warm-400 text-sm font-medium mb-1">
                  {stat.title}
                </h3>
                <p className="text-2xl font-bold text-white">
                  {stat.value}
                </p>
              </div>
            </div>
          )
        })}
      </div>

      {/* Additional Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {additionalStats.map((stat, index) => (
          <div key={index} className="card">
            <div className="flex items-center gap-4">
              <div className="p-2 bg-warm-600/20 rounded-lg">
                <stat.icon size={20} className="text-warm-400" />
              </div>
              <div>
                <h3 className="text-warm-400 text-sm font-medium">
                  {stat.title}
                </h3>
                <p className="text-xl font-semibold text-white">
                  {stat.value}
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Quick Actions */}
      <div className="card">
        <div className="card-header">
          <h2 className="text-xl font-semibold text-white">Quick Actions</h2>
        </div>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <button className="btn-primary">
            Start Trading
          </button>
          <button className="btn-outline">
            View Markets
          </button>
          <button className="btn-outline">
            Deposit Funds
          </button>
          <button className="btn-outline">
            View History
          </button>
        </div>
      </div>
    </div>
  )
}

export default Dashboard
