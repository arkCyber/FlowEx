#!/usr/bin/env python3
import http.server
import socketserver
import json
from datetime import datetime
import random
import time

MOCK_DATA = {
    'users': [{'id': '1', 'email': 'demo@flowex.com', 'firstName': 'Demo', 'lastName': 'User'}],
    'tradingPairs': [
        {'symbol': 'BTC-USDT', 'baseAsset': 'BTC', 'quoteAsset': 'USDT', 'status': 'TRADING'},
        {'symbol': 'ETH-USDT', 'baseAsset': 'ETH', 'quoteAsset': 'USDT', 'status': 'TRADING'}
    ],
    'balances': [
        {'currency': 'BTC', 'available': '0.12345678', 'locked': '0.00000000'},
        {'currency': 'USDT', 'available': '1000.00000000', 'locked': '50.00000000'}
    ]
}

class FlowExHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/health':
            self.send_json_response({'status': 'healthy', 'service': 'flowex-backend', 'timestamp': datetime.now().isoformat()})
        elif self.path == '/api/trading/pairs':
            self.send_json_response(MOCK_DATA['tradingPairs'])
        elif self.path == '/api/market-data/tickers':
            tickers = [{'symbol': p['symbol'], 'price': f'{random.uniform(30000, 50000):.2f}', 'change': f'{random.uniform(-5, 5):.2f}'} for p in MOCK_DATA['tradingPairs']]
            self.send_json_response(tickers)
        elif self.path == '/api/wallet/balances':
            self.send_json_response(MOCK_DATA['balances'])
        elif self.path == '/' or self.path == '/index.html':
            self.send_frontend()
        else:
            self.send_error(404)
    
    def do_POST(self):
        if self.path == '/api/auth/login':
            content_length = int(self.headers.get('Content-Length', 0))
            post_data = self.rfile.read(content_length)
            try:
                data = json.loads(post_data.decode('utf-8'))
                if data.get('email') == 'demo@flowex.com' and data.get('password') == 'demo123':
                    self.send_json_response({'token': f'mock_token_{int(time.time())}', 'user': MOCK_DATA['users'][0]})
                else:
                    self.send_error(401)
            except:
                self.send_error(400)
        else:
            self.send_error(404)
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, Authorization')
        self.end_headers()
    
    def send_json_response(self, data):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data).encode('utf-8'))
    
    def send_frontend(self):
        html = '''<!DOCTYPE html>
<html><head><title>FlowEx Enterprise</title>
<style>
body{font-family:Arial,sans-serif;margin:0;padding:20px;background:#f5f5f5}
.container{max-width:1200px;margin:0 auto}
.header{background:linear-gradient(135deg,#667eea,#764ba2);color:white;padding:40px;text-align:center;border-radius:10px;margin-bottom:30px}
.card{background:white;padding:30px;border-radius:10px;box-shadow:0 4px 6px rgba(0,0,0,0.1);margin-bottom:20px}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:20px}
.btn{background:#667eea;color:white;padding:12px 24px;border:none;border-radius:6px;cursor:pointer;margin:5px}
.btn:hover{background:#5a6fd8}
.status{background:#d4edda;color:#155724;padding:4px 12px;border-radius:20px;font-size:12px}
.endpoint{background:#f8f9fa;padding:10px;border-radius:4px;margin:5px 0;font-family:monospace}
.results{background:#f8f9fa;padding:15px;border-radius:4px;font-family:monospace;font-size:12px;max-height:300px;overflow-y:auto;margin-top:15px}
</style></head><body>
<div class="container">
<div class="header">
<h1>🚀 FlowEx Enterprise Trading Platform</h1>
<p>Complete enterprise-grade development environment</p>
<span class="status">Environment: Development</span>
</div>
<div class="grid">
<div class="card">
<h2>🌐 Services Status</h2>
<div>Backend API: <span class="status">Running on :8000</span></div>
<div>Frontend: <span class="status">Integrated</span></div>
<button class="btn" onclick="checkHealth()">Check Health</button>
<button class="btn" onclick="runAllTests()">Run All Tests</button>
</div>
<div class="card">
<h2>🧪 API Testing</h2>
<button class="btn" onclick="testLogin()">Test Login</button>
<button class="btn" onclick="testTradingPairs()">Test Trading Pairs</button>
<button class="btn" onclick="testTickers()">Test Market Data</button>
<button class="btn" onclick="testBalances()">Test Wallet</button>
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
<div style="background:#e7f3ff;padding:15px;border-radius:6px">
<strong>Email:</strong> demo@flowex.com<br>
<strong>Password:</strong> demo123
</div>
</div>
</div>
<div class="card">
<h2>📋 Test Results</h2>
<div id="results" class="results">Ready to run tests...</div>
</div>
</div>
<script>
let testCount = 0, passedTests = 0;
function log(msg, type = 'info') {
  const icon = type === 'success' ? '✅' : type === 'error' ? '❌' : 'ℹ️';
  document.getElementById('results').innerHTML += new Date().toLocaleTimeString() + ' ' + icon + ' ' + msg + '\\n';
}
async function makeRequest(url, options = {}) {
  try {
    const response = await fetch(url, options);
    const data = await response.json();
    return { success: true, status: response.status, data };
  } catch (error) {
    return { success: false, error: error.message };
  }
}
async function checkHealth() {
  log('Testing health endpoint...');
  const result = await makeRequest('/health');
  if (result.success && result.status === 200) {
    log('Health check passed: ' + result.data.status, 'success');
  } else {
    log('Health check failed: ' + (result.error || 'Unknown error'), 'error');
  }
}
async function testLogin() {
  log('Testing login endpoint...');
  const result = await makeRequest('/api/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email: 'demo@flowex.com', password: 'demo123' })
  });
  if (result.success && result.status === 200 && result.data.token) {
    log('Login test passed: Token received', 'success');
    passedTests++;
  } else {
    log('Login test failed: ' + (result.error || 'No token'), 'error');
  }
  testCount++;
}
async function testTradingPairs() {
  log('Testing trading pairs endpoint...');
  const result = await makeRequest('/api/trading/pairs');
  if (result.success && result.status === 200 && Array.isArray(result.data)) {
    log('Trading pairs test passed: ' + result.data.length + ' pairs found', 'success');
    passedTests++;
  } else {
    log('Trading pairs test failed: ' + (result.error || 'Invalid response'), 'error');
  }
  testCount++;
}
async function testTickers() {
  log('Testing market tickers endpoint...');
  const result = await makeRequest('/api/market-data/tickers');
  if (result.success && result.status === 200 && Array.isArray(result.data)) {
    log('Market tickers test passed: ' + result.data.length + ' tickers found', 'success');
    passedTests++;
  } else {
    log('Market tickers test failed: ' + (result.error || 'Invalid response'), 'error');
  }
  testCount++;
}
async function testBalances() {
  log('Testing wallet balances endpoint...');
  const result = await makeRequest('/api/wallet/balances');
  if (result.success && result.status === 200 && Array.isArray(result.data)) {
    log('Wallet balances test passed: ' + result.data.length + ' balances found', 'success');
    passedTests++;
  } else {
    log('Wallet balances test failed: ' + (result.error || 'Invalid response'), 'error');
  }
  testCount++;
}
async function runAllTests() {
  log('🚀 Starting comprehensive test suite...');
  testCount = 0; passedTests = 0;
  await checkHealth();
  await new Promise(resolve => setTimeout(resolve, 500));
  await testLogin();
  await new Promise(resolve => setTimeout(resolve, 500));
  await testTradingPairs();
  await new Promise(resolve => setTimeout(resolve, 500));
  await testTickers();
  await new Promise(resolve => setTimeout(resolve, 500));
  await testBalances();
  const successRate = testCount > 0 ? ((passedTests / testCount) * 100).toFixed(1) : 0;
  log('📊 Test Summary: ' + passedTests + '/' + testCount + ' tests passed (' + successRate + '% success rate)', 
      passedTests === testCount ? 'success' : 'error');
}
window.addEventListener('load', () => setTimeout(checkHealth, 1000));
</script></body></html>'''
        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(html.encode('utf-8'))

print('🚀 FlowEx Enterprise Environment Starting...')
print('==========================================')
with socketserver.TCPServer(('', 8000), FlowExHandler) as httpd:
    print('✅ Backend API Server: http://localhost:8000')
    print('✅ Frontend Interface: http://localhost:8000')
    print('✅ Health Check: http://localhost:8000/health')
    print('✅ API Base: http://localhost:8000/api')
    print('')
    print('📧 Demo Login: demo@flowex.com / demo123')
    print('')
    print('🎉 FlowEx Enterprise Environment Ready!')
    print('⏹️  Press Ctrl+C to stop')
    httpd.serve_forever()
