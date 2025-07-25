/**
 * FlowEx Main Layout Component with Tailwind CSS
 */

import React from 'react'
import { Sidebar } from './Sidebar'
import { Header } from './Header'
import { cn } from '../../utils/cn'

interface MainLayoutProps {
  children: React.ReactNode
  className?: string
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children, className }) => {
  return (
    <div className="flex min-h-screen bg-background-warm">
      {/* Sidebar */}
      <Sidebar />

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <Header />

        {/* Main Content */}
        <main className={cn(
          'flex-1 p-6 overflow-auto scrollbar-thin',
          className
        )}>
          <div className="max-w-7xl mx-auto">
            {children}
          </div>
        </main>
      </div>
    </div>
  )
}
