#!/bin/bash

# FlowEx Frontend Development Server
# Professional trading interface with hot reload

set -e

echo "🚀 Starting FlowEx Professional Trading Interface..."
echo "=================================================="

# Check if we're in the right directory
if [ ! -f "frontend/package.json" ]; then
    echo "❌ Error: Please run this script from the FlowEx root directory"
    exit 1
fi

# Navigate to frontend directory
cd frontend

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Check if Tailwind plugins are installed
echo "🎨 Checking Tailwind CSS plugins..."
npm list @tailwindcss/forms @tailwindcss/typography @tailwindcss/aspect-ratio 2>/dev/null || {
    echo "📦 Installing Tailwind CSS plugins..."
    npm install @tailwindcss/forms @tailwindcss/typography @tailwindcss/aspect-ratio
}

# Check if Heroicons is installed
npm list @heroicons/react 2>/dev/null || {
    echo "🎨 Installing Heroicons..."
    npm install @heroicons/react
}

echo ""
echo "🎯 FlowEx Trading Interface Features:"
echo "   • Professional dark theme with warm colors"
echo "   • Real-time trading charts and order book"
echo "   • Advanced order forms and position management"
echo "   • Responsive design for all devices"
echo "   • Inspired by Binance and OKX interfaces"
echo ""
echo "🌐 Starting development server..."
echo "   • Frontend: http://localhost:3000"
echo "   • Trading Page: http://localhost:3000/trading"
echo ""
echo "💡 Tips:"
echo "   • Use Ctrl+C to stop the server"
echo "   • The interface supports both light and dark themes"
echo "   • Default theme is dark mode with warm colors"
echo "   • All trading data is simulated for development"
echo ""

# Start the development server
npm start
