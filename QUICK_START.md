# 🚀 FlowEx Quick Start Guide

**Get FlowEx Trading Platform running in production in under 10 minutes!**

Created by **arkSong** (arksong2018@gmail.com) - FlowEx Founder

## 📋 Prerequisites

Before starting, ensure you have:

- **Docker 20.10+** installed and running
- **Docker Compose 2.0+** installed
- **4GB+ RAM** available
- **20GB+ disk space** for data and logs
- **Linux/macOS/Windows** with WSL2

## ⚡ Quick Deployment (Production Ready)

### Step 1: Clone and Setup
```bash
# Clone the repository
git clone https://github.com/arkCyber/FlowEx.git
cd FlowEx

# Make deployment script executable
chmod +x scripts/deploy-production.sh
```

### Step 2: Configure Environment
```bash
# Copy production environment template
cp .env.production .env

# Edit environment variables (IMPORTANT!)
nano .env  # or use your preferred editor
```

**⚠️ SECURITY CRITICAL**: Update these variables in `.env`:
```bash
POSTGRES_PASSWORD=your_secure_postgres_password_here
REDIS_PASSWORD=your_secure_redis_password_here
JWT_SECRET=your_very_secure_jwt_secret_minimum_32_characters_long
GRAFANA_PASSWORD=your_secure_grafana_password_here
```

### Step 3: Deploy FlowEx
```bash
# Deploy the entire platform
./scripts/deploy-production.sh deploy

# This will:
# ✅ Validate prerequisites
# ✅ Check environment configuration
# ✅ Create database backup
# ✅ Deploy all services
# ✅ Run database migrations
# ✅ Perform health checks
```

### Step 4: Verify Deployment
```bash
# Check all services status
./scripts/deploy-production.sh status

# Run health checks
./scripts/deploy-production.sh health

# View logs (optional)
./scripts/deploy-production.sh logs
```

## 🌐 Access Your FlowEx Platform

Once deployed, access these endpoints:

| Service | URL | Description |
|---------|-----|-------------|
| **Main Platform** | http://localhost:8000 | API Gateway (main entry point) |
| **Authentication** | http://localhost:8001 | User authentication service |
| **Trading** | http://localhost:8002 | Trading and order management |
| **Market Data** | http://localhost:8003 | Real-time market data |
| **Wallet** | http://localhost:8004 | Balance and transaction management |
| **Grafana Dashboard** | http://localhost:3000 | Monitoring and analytics |
| **Prometheus Metrics** | http://localhost:9090 | Metrics collection |

## 🧪 Test the Platform

### Demo User Credentials
```
Email: demo@flowex.com
Password: demo123
```

### API Testing Examples

#### 1. Health Check
```bash
curl http://localhost:8000/health
```

#### 2. User Authentication
```bash
# Login
curl -X POST http://localhost:8001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"demo@flowex.com","password":"demo123"}'

# Get user profile (replace TOKEN with JWT from login)
curl http://localhost:8001/api/auth/me \
  -H "Authorization: Bearer TOKEN"
```

#### 3. Trading Operations
```bash
# Get trading pairs
curl http://localhost:8002/api/trading/pairs

# Get order book
curl http://localhost:8002/api/trading/orderbook/BTCUSDT

# Create order (requires authentication)
curl -X POST http://localhost:8002/api/trading/orders \
  -H "Authorization: Bearer TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "trading_pair": "BTCUSDT",
    "side": "buy",
    "order_type": "limit",
    "price": "43000.00",
    "quantity": "0.001"
  }'
```

#### 4. Market Data
```bash
# Get ticker data
curl http://localhost:8003/api/market-data/ticker/BTCUSDT

# Get all tickers
curl http://localhost:8003/api/market-data/tickers

# Get recent trades
curl http://localhost:8003/api/market-data/trades/BTCUSDT
```

#### 5. Wallet Operations
```bash
# Get all balances (requires authentication)
curl http://localhost:8004/api/wallet/balances \
  -H "Authorization: Bearer TOKEN"

# Get specific balance
curl http://localhost:8004/api/wallet/balance/BTC \
  -H "Authorization: Bearer TOKEN"

# Get transaction history
curl http://localhost:8004/api/wallet/transactions \
  -H "Authorization: Bearer TOKEN"
```

## 📊 Monitoring & Analytics

### Grafana Dashboard
1. Open http://localhost:3000
2. Login with:
   - Username: `admin`
   - Password: `your_grafana_password` (from .env file)
3. Explore pre-configured dashboards for:
   - Service performance metrics
   - Trading volume and activity
   - System resource usage
   - Error rates and health status

### Prometheus Metrics
- Access raw metrics at http://localhost:9090
- Query custom business metrics
- Set up alerts and monitoring rules

## 🔧 Management Commands

### Service Management
```bash
# Update deployment
./scripts/deploy-production.sh update

# Check status
./scripts/deploy-production.sh status

# View logs for specific service
./scripts/deploy-production.sh logs trading-service

# View all logs
./scripts/deploy-production.sh logs
```

### Backup & Recovery
```bash
# Create database backup
./scripts/deploy-production.sh backup

# Rollback to previous version
./scripts/deploy-production.sh rollback
```

### Health Monitoring
```bash
# Run comprehensive health checks
./scripts/deploy-production.sh health
```

## 🛠️ Development Mode

For development and testing:

```bash
# Use development environment
npm run dev

# This starts:
# ✅ All backend services
# ✅ Frontend development server
# ✅ Database and Redis
# ✅ Hot reload for development
```

## 🔒 Security Best Practices

### Production Security Checklist
- ✅ Change all default passwords in `.env`
- ✅ Use strong JWT secrets (32+ characters)
- ✅ Enable HTTPS with SSL certificates
- ✅ Configure firewall rules
- ✅ Set up proper backup procedures
- ✅ Monitor logs for security events
- ✅ Regular security updates

### SSL/HTTPS Setup
```bash
# Generate SSL certificates (example with Let's Encrypt)
certbot certonly --standalone -d yourdomain.com

# Update nginx configuration
# Copy certificates to ./nginx/ssl/
# Update CORS_ALLOWED_ORIGINS in .env
```

## 🚨 Troubleshooting

### Common Issues

#### Services not starting
```bash
# Check Docker status
docker info

# Check logs
./scripts/deploy-production.sh logs

# Restart services
docker-compose -f docker-compose.production.yml restart
```

#### Database connection issues
```bash
# Check PostgreSQL health
docker-compose -f docker-compose.production.yml exec postgres pg_isready -U flowex

# Reset database (⚠️ DATA LOSS)
docker-compose -f docker-compose.production.yml down -v
./scripts/deploy-production.sh deploy
```

#### Memory issues
```bash
# Check resource usage
docker stats

# Increase Docker memory limits
# Restart Docker daemon
```

### Getting Help

- 📧 **Email**: arksong2018@gmail.com
- 🐛 **Issues**: https://github.com/arkCyber/FlowEx/issues
- 💬 **Discussions**: https://github.com/arkCyber/FlowEx/discussions
- 📖 **Documentation**: See README.md and README_CN.md

## 🎉 Success!

Congratulations! You now have a fully functional, enterprise-grade cryptocurrency trading platform running in production.

### What's Next?

1. **Customize** trading pairs and fees
2. **Configure** external integrations
3. **Set up** monitoring alerts
4. **Scale** services based on load
5. **Implement** additional security measures

### Support the Project

If FlowEx helps your business:
- ⭐ **Star** the repository on GitHub
- 🍴 **Fork** and contribute improvements
- 📢 **Share** with your network
- 💝 **Sponsor** the development

---

**Built with ❤️ by arkSong and the FlowEx community**

🚀 **Ready to revolutionize trading? Let's build the future together!**
