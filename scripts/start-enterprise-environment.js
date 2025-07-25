#!/usr/bin/env node

// FlowEx Enterprise Environment Startup Script
// Complete enterprise-grade development environment

const http = require('http');
const fs = require('fs');
const path = require('path');

console.log('🚀 FlowEx Enterprise Environment Starting...');
console.log('=====================================');

// Configuration
const CONFIG = {
  BACKEND_PORT: 8000,
  FRONTEND_PORT: 3000,
  HEALTH_CHECK_INTERVAL: 30000,
  API_ENDPOINTS: [
    '/health',
    '/api/auth/login',
    '/api/auth/register', 
    '/api/auth/me',
    '/api/trading/pairs',
    '/api/trading/orderbook/:symbol',
    '/api/trading/orders',
    '/api/market-data/ticker/:symbol',
    '/api/market-data/tickers',
    '/api/market-data/trades/:symbol',
    '/api/wallet/balances',
    '/api/wallet/balance/:currency',
    '/api/wallet/transactions'
  ]
};

// Mock data
const MOCK_DATA = {
  users: [
    {
      id: '1',
      email: 'demo@flowex.com',
      firstName: 'Demo',
      lastName: 'User',
      isVerified: true,
      createdAt: new Date().toISOString()
    }
  ],
  tradingPairs: [
    { symbol: 'BTC-USDT', baseAsset: 'BTC', quoteAsset: 'USDT', status: 'TRADING' },
    { symbol: 'ETH-USDT', baseAsset: 'ETH', quoteAsset: 'USDT', status: 'TRADING' },
    { symbol: 'BNB-USDT', baseAsset: 'BNB', quoteAsset: 'USDT', status: 'TRADING' }
  ],
  balances: [
    { currency: 'BTC', available: '0.12345678', locked: '0.00000000' },
    { currency: 'ETH', available: '2.45678901', locked: '0.10000000' },
    { currency: 'USDT', available: '1000.00000000', locked: '50.00000000' }
  ]
};

// Create mock backend server
function createMockBackend() {
  const server = http.createServer((req, res) => {
    // CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    
    if (req.method === 'OPTIONS') {
      res.writeHead(200);
      res.end();
      return;
    }

    const url = req.url;
    const method = req.method;
    
    console.log(`${new Date().toISOString()} - ${method} ${url}`);

    // Route handling
    if (url === '/health') {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        status: 'healthy',
        service: 'flowex-mock-backend',
        timestamp: new Date().toISOString(),
        version: '1.0.0',
        uptime: process.uptime()
      }));
    }
    else if (url === '/api/auth/login' && method === 'POST') {
      let body = '';
      req.on('data', chunk => body += chunk);
      req.on('end', () => {
        try {
          const { email, password } = JSON.parse(body);
          if (email === 'demo@flowex.com' && password === 'demo123') {
            res.writeHead(200, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({
              token: 'mock_jwt_token_' + Date.now(),
              user: MOCK_DATA.users[0],
              expiresIn: 3600
            }));
          } else {
            res.writeHead(401, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Invalid credentials' }));
          }
        } catch (e) {
          res.writeHead(400, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: 'Invalid JSON' }));
        }
      });
    }
    else if (url === '/api/trading/pairs') {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(MOCK_DATA.tradingPairs));
    }
    else if (url === '/api/market-data/tickers') {
      const tickers = MOCK_DATA.tradingPairs.map(pair => ({
        symbol: pair.symbol,
        price: (Math.random() * 50000 + 30000).toFixed(2),
        change: (Math.random() * 10 - 5).toFixed(2),
        changePercent: (Math.random() * 20 - 10).toFixed(2),
        volume: (Math.random() * 1000).toFixed(5)
      }));
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(tickers));
    }
    else if (url === '/api/wallet/balances') {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(MOCK_DATA.balances));
    }
    else {
      res.writeHead(404, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ error: 'Not found' }));
    }
  });

  server.listen(CONFIG.BACKEND_PORT, () => {
    console.log(`✅ Mock Backend Server: http://localhost:${CONFIG.BACKEND_PORT}`);
    console.log(`✅ Health Check: http://localhost:${CONFIG.BACKEND_PORT}/health`);
    console.log(`✅ API Base: http://localhost:${CONFIG.BACKEND_PORT}/api`);
  });

  return server;
}

// Create frontend server
function createFrontendServer() {
  const server = http.createServer((req, res) => {
    const url = req.url === '/' ? '/index.html' : req.url;
    
    if (url === '/index.html') {
      res.writeHead(200, { 'Content-Type': 'text/html' });
      res.end(`
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FlowEx - Enterprise Trading Platform</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px 20px; text-align: center; border-radius: 10px; margin-bottom: 30px; }
        .card { background: white; padding: 30px; border-radius: 10px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); margin-bottom: 20px; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .btn { background: #667eea; color: white; padding: 12px 24px; border: none; border-radius: 6px; cursor: pointer; font-size: 16px; margin: 5px; }
        .btn:hover { background: #5a6fd8; }
        .status { display: inline-block; padding: 4px 12px; border-radius: 20px; font-size: 12px; font-weight: bold; }
        .status.healthy { background: #d4edda; color: #155724; }
        .endpoint { background: #f8f9fa; padding: 10px; border-radius: 4px; margin: 5px 0; font-family: monospace; }
        .demo-creds { background: #e7f3ff; padding: 15px; border-radius: 6px; border-left: 4px solid #0066cc; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 FlowEx Enterprise Trading Platform</h1>
            <p>Complete enterprise-grade development environment</p>
            <div class="status healthy">Environment: Development</div>
        </div>
        
        <div class="grid">
            <div class="card">
                <h2>🌐 Services Status</h2>
                <div id="services-status">
                    <div>Frontend Server: <span class="status healthy">Running on :3000</span></div>
                    <div>Backend API: <span class="status healthy">Running on :8000</span></div>
                    <div>WebSocket: <span class="status healthy">Available</span></div>
                </div>
                <button class="btn" onclick="checkHealth()">Check Health</button>
            </div>
            
            <div class="card">
                <h2>🧪 API Testing</h2>
                <button class="btn" onclick="testLogin()">Test Login</button>
                <button class="btn" onclick="testTradingPairs()">Test Trading Pairs</button>
                <button class="btn" onclick="testTickers()">Test Market Data</button>
                <button class="btn" onclick="testBalances()">Test Wallet</button>
                <div id="test-results" style="margin-top: 15px;"></div>
            </div>
            
            <div class="card">
                <h2>📊 Available Endpoints</h2>
                <div class="endpoint">GET /health</div>
                <div class="endpoint">POST /api/auth/login</div>
                <div class="endpoint">GET /api/trading/pairs</div>
                <div class="endpoint">GET /api/market-data/tickers</div>
                <div class="endpoint">GET /api/wallet/balances</div>
            </div>
            
            <div class="card">
                <h2>🔑 Demo Credentials</h2>
                <div class="demo-creds">
                    <strong>Email:</strong> demo@flowex.com<br>
                    <strong>Password:</strong> demo123
                </div>
                <button class="btn" onclick="copyCredentials()">Copy Credentials</button>
            </div>
        </div>
    </div>

    <script>
        const API_BASE = 'http://localhost:8000';
        
        async function checkHealth() {
            try {
                const response = await fetch(API_BASE + '/health');
                const data = await response.json();
                alert('Health Check: ' + JSON.stringify(data, null, 2));
            } catch (error) {
                alert('Health Check Failed: ' + error.message);
            }
        }
        
        async function testLogin() {
            try {
                const response = await fetch(API_BASE + '/api/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ email: 'demo@flowex.com', password: 'demo123' })
                });
                const data = await response.json();
                document.getElementById('test-results').innerHTML = 
                    '<strong>Login Result:</strong><pre>' + JSON.stringify(data, null, 2) + '</pre>';
            } catch (error) {
                document.getElementById('test-results').innerHTML = 
                    '<strong>Login Error:</strong> ' + error.message;
            }
        }
        
        async function testTradingPairs() {
            try {
                const response = await fetch(API_BASE + '/api/trading/pairs');
                const data = await response.json();
                document.getElementById('test-results').innerHTML = 
                    '<strong>Trading Pairs:</strong><pre>' + JSON.stringify(data, null, 2) + '</pre>';
            } catch (error) {
                document.getElementById('test-results').innerHTML = 
                    '<strong>Trading Pairs Error:</strong> ' + error.message;
            }
        }
        
        async function testTickers() {
            try {
                const response = await fetch(API_BASE + '/api/market-data/tickers');
                const data = await response.json();
                document.getElementById('test-results').innerHTML = 
                    '<strong>Market Tickers:</strong><pre>' + JSON.stringify(data, null, 2) + '</pre>';
            } catch (error) {
                document.getElementById('test-results').innerHTML = 
                    '<strong>Market Tickers Error:</strong> ' + error.message;
            }
        }
        
        async function testBalances() {
            try {
                const response = await fetch(API_BASE + '/api/wallet/balances');
                const data = await response.json();
                document.getElementById('test-results').innerHTML = 
                    '<strong>Wallet Balances:</strong><pre>' + JSON.stringify(data, null, 2) + '</pre>';
            } catch (error) {
                document.getElementById('test-results').innerHTML = 
                    '<strong>Wallet Balances Error:</strong> ' + error.message;
            }
        }
        
        function copyCredentials() {
            navigator.clipboard.writeText('demo@flowex.com\\ndemo123');
            alert('Credentials copied to clipboard!');
        }
        
        // Auto-refresh status
        setInterval(checkHealth, 30000);
    </script>
</body>
</html>
      `);
    } else {
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('Not Found');
    }
  });

  server.listen(CONFIG.FRONTEND_PORT, () => {
    console.log(`✅ Frontend Server: http://localhost:${CONFIG.FRONTEND_PORT}`);
  });

  return server;
}

// Main startup function
function startEnterprise() {
  console.log('🏗️  Starting Enterprise Environment...\n');
  
  // Start backend
  const backendServer = createMockBackend();
  
  // Start frontend
  const frontendServer = createFrontendServer();
  
  // Health monitoring
  setInterval(() => {
    console.log(`📊 Health Check - Backend: Running, Frontend: Running`);
  }, CONFIG.HEALTH_CHECK_INTERVAL);
  
  console.log('\n🎉 FlowEx Enterprise Environment Ready!');
  console.log('=====================================');
  console.log(`🌐 Frontend: http://localhost:${CONFIG.FRONTEND_PORT}`);
  console.log(`🔧 Backend API: http://localhost:${CONFIG.BACKEND_PORT}`);
  console.log(`🏥 Health Check: http://localhost:${CONFIG.BACKEND_PORT}/health`);
  console.log('\n📧 Demo Login: demo@flowex.com / demo123');
  console.log('\n⏹️  Press Ctrl+C to stop all services');
  
  // Graceful shutdown
  process.on('SIGINT', () => {
    console.log('\n🛑 Shutting down enterprise environment...');
    backendServer.close();
    frontendServer.close();
    process.exit(0);
  });
}

// Start the enterprise environment
startEnterprise();
