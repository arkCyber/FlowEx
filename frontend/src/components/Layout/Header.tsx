/**
 * FlowEx Header Component
 */

import React from 'react'
import { Bell, Search, User, Menu } from 'lucide-react'
import { CompactThemeToggle } from '../ThemeToggle'
import { useAuth } from '../../hooks/useAuth'
import { cn } from '../../utils/cn'

interface HeaderProps {
  className?: string
}

export const Header: React.FC<HeaderProps> = ({ className }) => {
  const { user } = useAuth()

  return (
    <header className={cn(
      'bg-background-warm-secondary border-b border-warm-800 px-6 py-4',
      className
    )}>
      <div className="flex items-center justify-between">
        {/* Left Section */}
        <div className="flex items-center gap-4">
          {/* Mobile Menu Button */}
          <button className="lg:hidden p-2 rounded-lg hover:bg-warm-800 transition-colors">
            <Menu size={20} className="text-warm-400" />
          </button>
          
          {/* Search */}
          <div className="hidden md:flex items-center gap-2 bg-background-warm-tertiary rounded-lg px-3 py-2 min-w-80">
            <Search size={18} className="text-warm-500" />
            <input
              type="text"
              placeholder="Search markets, assets..."
              className="bg-transparent text-white placeholder-warm-500 outline-none flex-1"
            />
            <kbd className="hidden lg:inline-block px-2 py-1 text-xs font-mono bg-warm-700 text-warm-300 rounded">
              ⌘K
            </kbd>
          </div>
        </div>

        {/* Right Section */}
        <div className="flex items-center gap-3">
          {/* Theme Toggle */}
          <CompactThemeToggle />
          
          {/* Notifications */}
          <button className="relative p-2 rounded-lg hover:bg-warm-800 transition-colors">
            <Bell size={20} className="text-warm-400" />
            <span className="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full"></span>
          </button>
          
          {/* User Menu */}
          <div className="flex items-center gap-3">
            <div className="hidden sm:block text-right">
              <div className="text-sm font-medium text-white">
                {user?.firstName} {user?.lastName}
              </div>
              <div className="text-xs text-warm-400">
                {user?.email}
              </div>
            </div>
            
            <button className="flex items-center gap-2 p-2 rounded-lg hover:bg-warm-800 transition-colors">
              <div className="w-8 h-8 bg-primary-600 rounded-full flex items-center justify-center">
                <User size={16} className="text-white" />
              </div>
            </button>
          </div>
        </div>
      </div>
    </header>
  )
}
