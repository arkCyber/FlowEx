# FlowEx Frontend - Multi-stage Docker Build
# Optimized for production deployment with security and performance

# Build stage
FROM node:18-alpine as builder

# Set working directory
WORKDIR /app

# Install dependencies for native modules
RUN apk add --no-cache python3 make g++

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production --silent

# Copy source code
COPY . .

# Build arguments
ARG VITE_API_BASE_URL=http://localhost:8001
ARG VITE_WS_BASE_URL=ws://localhost:8001
ARG VITE_APP_VERSION=1.0.0
ARG VITE_ENVIRONMENT=production

# Set environment variables
ENV VITE_API_BASE_URL=$VITE_API_BASE_URL
ENV VITE_WS_BASE_URL=$VITE_WS_BASE_URL
ENV VITE_APP_VERSION=$VITE_APP_VERSION
ENV VITE_ENVIRONMENT=$VITE_ENVIRONMENT

# Build the application
RUN npm run build

# Production stage
FROM nginx:alpine

# Install security updates
RUN apk update && apk upgrade && apk add --no-cache curl

# Create non-root user
RUN addgroup -g 1001 -S flowex && \
    adduser -S flowex -u 1001

# Copy built application
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy environment variable substitution script
COPY docker-entrypoint.sh /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh

# Set ownership
RUN chown -R flowex:flowex /usr/share/nginx/html && \
    chown -R flowex:flowex /var/cache/nginx && \
    chown -R flowex:flowex /var/log/nginx && \
    chown -R flowex:flowex /etc/nginx/conf.d

# Create nginx pid directory
RUN mkdir -p /var/run/nginx && \
    chown -R flowex:flowex /var/run/nginx

# Switch to non-root user
USER flowex

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:80/health || exit 1

# Expose port
EXPOSE 80

# Start nginx
ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]
