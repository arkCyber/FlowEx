#!/bin/sh

# FlowEx Frontend Docker Entrypoint
# Handles environment variable substitution and nginx startup

set -e

echo "🚀 Starting FlowEx Frontend"
echo "Environment: ${VITE_ENVIRONMENT:-production}"
echo "API URL: ${VITE_API_BASE_URL:-http://localhost:8001}"

# Substitute environment variables in index.html
if [ -f "/usr/share/nginx/html/index.html" ]; then
    echo "📝 Substituting environment variables..."
    
    # Replace placeholders with actual environment variables
    sed -i "s|%VITE_API_BASE_URL%|${VITE_API_BASE_URL:-http://localhost:8001}|g" /usr/share/nginx/html/index.html
    sed -i "s|%VITE_WS_BASE_URL%|${VITE_WS_BASE_URL:-ws://localhost:8001}|g" /usr/share/nginx/html/index.html
    sed -i "s|%VITE_APP_VERSION%|${VITE_APP_VERSION:-1.0.0}|g" /usr/share/nginx/html/index.html
    sed -i "s|%VITE_BUILD_TIME%|$(date -u +%Y-%m-%dT%H:%M:%SZ)|g" /usr/share/nginx/html/index.html
    sed -i "s|%VITE_ENVIRONMENT%|${VITE_ENVIRONMENT:-production}|g" /usr/share/nginx/html/index.html
    
    echo "✅ Environment variables substituted"
fi

# Test nginx configuration
echo "🔧 Testing nginx configuration..."
nginx -t

echo "✅ FlowEx Frontend ready to start"

# Execute the main command
exec "$@"
