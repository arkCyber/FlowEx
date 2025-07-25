/**
 * FlowEx Sidebar Component
 */

import React from 'react'
import { NavLink } from 'react-router-dom'
import {
  LayoutDashboard,
  TrendingUp,
  Wallet,
  BarChart3,
  ShoppingCart,
  Settings,
  User,
  LogOut,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react'
import { useAuth } from '../../hooks/useAuth'
import { cn } from '../../utils/cn'

interface SidebarProps {
  className?: string
}

const navigation = [
  {
    name: 'Dashboard',
    href: '/dashboard',
    icon: LayoutDashboard,
  },
  {
    name: 'Trading',
    href: '/trading',
    icon: TrendingUp,
  },
  {
    name: 'Markets',
    href: '/markets',
    icon: BarChart3,
  },
  {
    name: 'Portfolio',
    href: '/portfolio',
    icon: Wallet,
  },
  {
    name: 'Orders',
    href: '/orders',
    icon: ShoppingCart,
  },
  {
    name: 'Profile',
    href: '/profile',
    icon: User,
  },
  {
    name: 'Settings',
    href: '/settings',
    icon: Settings,
  },
]

export const Sidebar: React.FC<SidebarProps> = ({ className }) => {
  const { logout } = useAuth()
  const [isCollapsed, setIsCollapsed] = React.useState(false)

  return (
    <aside className={cn(
      'bg-background-warm-secondary border-r border-warm-800 transition-all duration-300',
      isCollapsed ? 'w-16' : 'w-64',
      className
    )}>
      <div className="flex flex-col h-full">
        {/* Logo */}
        <div className="p-6 border-b border-warm-800">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
              <span className="text-white font-bold text-sm">F</span>
            </div>
            {!isCollapsed && (
              <div>
                <h1 className="text-xl font-bold text-white">FlowEx</h1>
                <p className="text-xs text-warm-400">Trading Platform</p>
              </div>
            )}
          </div>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-4">
          <ul className="space-y-2">
            {navigation.map((item) => (
              <li key={item.name}>
                <NavLink
                  to={item.href}
                  className={({ isActive }) =>
                    cn(
                      'flex items-center gap-3 px-3 py-2 rounded-lg transition-all duration-200 group',
                      isActive
                        ? 'bg-primary-600 text-white shadow-lg'
                        : 'text-warm-400 hover:text-white hover:bg-warm-800',
                      isCollapsed && 'justify-center'
                    )
                  }
                  title={isCollapsed ? item.name : undefined}
                >
                  <item.icon size={20} className="flex-shrink-0" />
                  {!isCollapsed && (
                    <span className="font-medium">{item.name}</span>
                  )}
                </NavLink>
              </li>
            ))}
          </ul>
        </nav>

        {/* Bottom Actions */}
        <div className="p-4 border-t border-warm-800">
          <button
            onClick={() => logout()}
            className={cn(
              'flex items-center gap-3 px-3 py-2 rounded-lg transition-all duration-200 text-warm-400 hover:text-white hover:bg-red-600 w-full',
              isCollapsed && 'justify-center'
            )}
            title={isCollapsed ? 'Logout' : undefined}
          >
            <LogOut size={20} className="flex-shrink-0" />
            {!isCollapsed && <span className="font-medium">Logout</span>}
          </button>
        </div>

        {/* Collapse Toggle */}
        <button
          onClick={() => setIsCollapsed(!isCollapsed)}
          className="absolute -right-3 top-20 w-6 h-6 bg-background-warm-secondary border border-warm-800 rounded-full flex items-center justify-center hover:bg-warm-800 transition-colors"
        >
          {isCollapsed ? (
            <ChevronRight size={14} className="text-warm-400" />
          ) : (
            <ChevronLeft size={14} className="text-warm-400" />
          )}
        </button>
      </div>
    </aside>
  )
}
