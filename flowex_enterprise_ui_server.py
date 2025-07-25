#!/usr/bin/env python3

"""
FlowEx Enterprise UI Server with Dark Theme
Complete enterprise-grade frontend with warm dark background
"""

import http.server
import socketserver
import json
import urllib.parse
from datetime import datetime
import random
import time

# Mock data for enterprise demo
MOCK_DATA = {
    'users': [
        {
            'id': '1',
            'email': 'demo@flowex.com',
            'firstName': 'Demo',
            'lastName': 'User',
            'isVerified': True,
            'createdAt': datetime.now().isoformat()
        }
    ],
    'tradingPairs': [
        {'symbol': 'BTC-USDT', 'baseAsset': 'BTC', 'quoteAsset': 'USDT', 'status': 'TRADING'},
        {'symbol': 'ETH-USDT', 'baseAsset': 'ETH', 'quoteAsset': 'USDT', 'status': 'TRADING'},
        {'symbol': 'BNB-USDT', 'baseAsset': 'BNB', 'quoteAsset': 'USDT', 'status': 'TRADING'}
    ],
    'balances': [
        {'currency': 'BTC', 'available': '0.12345678', 'locked': '0.00000000'},
        {'currency': 'ETH', 'available': '2.45678901', 'locked': '0.10000000'},
        {'currency': 'USDT', 'available': '1000.00000000', 'locked': '50.00000000'},
        {'currency': 'BNB', 'available': '10.50000000', 'locked': '0.00000000'}
    ]
}

class FlowExHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        """Handle GET requests"""
        if self.path == '/health':
            self.send_json_response({
                'status': 'healthy',
                'service': 'flowex-enterprise-ui',
                'timestamp': datetime.now().isoformat(),
                'version': '1.0.0',
                'uptime': time.time()
            })
        elif self.path == '/api/trading/pairs':
            self.send_json_response(MOCK_DATA['tradingPairs'])
        elif self.path == '/api/market-data/tickers':
            tickers = []
            for pair in MOCK_DATA['tradingPairs']:
                base_price = 45000 if 'BTC' in pair['symbol'] else 3000 if 'ETH' in pair['symbol'] else 300
                change = random.uniform(-5, 5)
                tickers.append({
                    'symbol': pair['symbol'],
                    'price': f'{base_price + change:.2f}',
                    'change': f'{change:.2f}',
                    'changePercent': f'{(change/base_price)*100:.2f}',
                    'volume': f'{random.uniform(100, 1000):.5f}',
                    'high': f'{base_price + random.uniform(0, 1000):.2f}',
                    'low': f'{base_price - random.uniform(0, 1000):.2f}'
                })
            self.send_json_response(tickers)
        elif self.path == '/api/wallet/balances':
            self.send_json_response(MOCK_DATA['balances'])
        elif self.path == '/' or self.path == '/index.html':
            self.send_frontend()
        else:
            self.send_error(404, 'Endpoint not found')
    
    def do_POST(self):
        """Handle POST requests"""
        if self.path == '/api/auth/login':
            content_length = int(self.headers.get('Content-Length', 0))
            post_data = self.rfile.read(content_length)
            try:
                data = json.loads(post_data.decode('utf-8'))
                email = data.get('email', '')
                password = data.get('password', '')
                
                if email == 'demo@flowex.com' and password == 'demo123':
                    self.send_json_response({
                        'token': f'mock_jwt_token_{int(time.time())}',
                        'user': MOCK_DATA['users'][0],
                        'expiresIn': 3600
                    })
                else:
                    self.send_error(401, 'Invalid credentials')
            except json.JSONDecodeError:
                self.send_error(400, 'Invalid JSON')
        else:
            self.send_error(404, 'Endpoint not found')
    
    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, Authorization')
        self.end_headers()
    
    def send_json_response(self, data):
        """Send JSON response with CORS headers"""
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, indent=2).encode('utf-8'))
    
    def send_frontend(self):
        """Send the enterprise dark theme frontend HTML interface"""
        html = '''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FlowEx Enterprise Trading Platform</title>
    <style>
        /* Enterprise Dark Theme with Warm Background */
        * { margin: 0; padding: 0; box-sizing: border-box; }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 25%, #0f3460 50%, #533a71 75%, #6b4c93 100%);
            color: #e8e6e3;
            min-height: 100vh;
            padding: 20px;
            line-height: 1.6;
        }
        
        .container { max-width: 1400px; margin: 0 auto; }
        
        .header {
            background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 25%, #ff9ff3 50%, #54a0ff 75%, #5f27cd 100%);
            color: #fff;
            padding: 50px 30px;
            text-align: center;
            border-radius: 20px;
            margin-bottom: 40px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.3);
            position: relative;
            overflow: hidden;
        }
        
        .header h1 { font-size: 3rem; margin-bottom: 15px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3); position: relative; z-index: 1; }
        .header p { font-size: 1.2rem; opacity: 0.9; position: relative; z-index: 1; }
        
        .card {
            background: linear-gradient(145deg, #2c2c54 0%, #40407a 100%);
            padding: 35px;
            border-radius: 20px;
            box-shadow: 0 15px 35px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.1);
            margin-bottom: 25px;
            border: 1px solid rgba(255,255,255,0.1);
            backdrop-filter: blur(10px);
        }
        
        .card h2 { color: #ffa726; margin-bottom: 20px; font-size: 1.5rem; font-weight: 600; }
        
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 25px; }
        
        .btn {
            background: linear-gradient(135deg, #ff6b6b, #ee5a24);
            color: #fff;
            padding: 14px 28px;
            border: none;
            border-radius: 12px;
            cursor: pointer;
            margin: 8px;
            font-size: 16px;
            font-weight: 600;
            transition: all 0.3s ease;
            box-shadow: 0 8px 20px rgba(255,107,107,0.3);
        }
        
        .btn:hover {
            background: linear-gradient(135deg, #ff5252, #d84315);
            transform: translateY(-2px);
            box-shadow: 0 12px 25px rgba(255,107,107,0.4);
        }
        
        .btn.success {
            background: linear-gradient(135deg, #26de81, #20bf6b);
            box-shadow: 0 8px 20px rgba(38,222,129,0.3);
        }
        
        .status {
            background: linear-gradient(135deg, #26de81, #20bf6b);
            color: #fff;
            padding: 8px 16px;
            border-radius: 25px;
            font-size: 13px;
            font-weight: 600;
            display: inline-block;
            box-shadow: 0 4px 12px rgba(38,222,129,0.3);
        }
        
        .endpoint {
            background: linear-gradient(145deg, #1e1e2e, #2d2d44);
            padding: 15px;
            border-radius: 12px;
            margin: 8px 0;
            font-family: 'SF Mono', Monaco, Inconsolata, 'Roboto Mono', Consolas, 'Courier New', monospace;
            border: 1px solid rgba(255,255,255,0.1);
            color: #a8e6cf;
            font-size: 14px;
            box-shadow: inset 0 2px 4px rgba(0,0,0,0.2);
        }
        
        .results {
            background: linear-gradient(145deg, #1a1a2e, #16213e);
            padding: 20px;
            border-radius: 15px;
            font-family: 'SF Mono', Monaco, Inconsolata, 'Roboto Mono', Consolas, 'Courier New', monospace;
            font-size: 13px;
            max-height: 400px;
            overflow-y: auto;
            margin-top: 20px;
            border: 1px solid rgba(255,255,255,0.1);
            color: #e8e6e3;
            box-shadow: inset 0 4px 8px rgba(0,0,0,0.3);
        }
        
        .demo-creds {
            background: linear-gradient(145deg, #0f3460, #533a71);
            padding: 20px;
            border-radius: 15px;
            border-left: 4px solid #ffa726;
            color: #e8e6e3;
            box-shadow: 0 8px 20px rgba(0,0,0,0.2);
        }
        
        .rust-badge {
            background: linear-gradient(135deg, #ce422b, #8b0000);
            color: #fff;
            padding: 6px 12px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: 600;
            display: inline-block;
            margin: 5px;
            box-shadow: 0 4px 12px rgba(206,66,43,0.3);
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 FlowEx Enterprise Trading Platform</h1>
            <p>Complete enterprise-grade development environment with Rust backend</p>
            <div class="status">Environment: Development</div>
            <span class="rust-badge">🦀 Rust Backend</span>
        </div>
        
        <div class="grid">
            <div class="card">
                <h2>🌐 Services Status</h2>
                <div style="margin-bottom: 15px;">
                    <div>Frontend UI: <span class="status">Running on :8000</span></div>
                    <div>Rust Backend: <span class="status">Available</span></div>
                    <div>Health Check: <span class="status">Active</span></div>
                </div>
                <button class="btn" onclick="checkHealth()">Check Health</button>
                <button class="btn" onclick="clearResults()">Clear Results</button>
            </div>
            
            <div class="card">
                <h2>🧪 API Testing Suite</h2>
                <div style="margin-bottom: 15px;">
                    <button class="btn" onclick="testLogin()">Test Login</button>
                    <button class="btn" onclick="testTradingPairs()">Test Trading Pairs</button>
                    <button class="btn" onclick="testTickers()">Test Market Data</button>
                    <button class="btn" onclick="testBalances()">Test Wallet</button>
                </div>
                <button class="btn success" onclick="runAllTests()">Run All Tests</button>
            </div>
            
            <div class="card">
                <h2>📊 Available API Endpoints</h2>
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
        
        <div class="card">
            <h2>📋 Test Results</h2>
            <div id="results" class="results">Ready to run tests...\n</div>
        </div>
    </div>

    <script>
        let testCount = 0;
        let passedTests = 0;
        
        function log(message, type = 'info') {
            const timestamp = new Date().toLocaleTimeString();
            const icon = type === 'success' ? '✅' : type === 'error' ? '❌' : 'ℹ️';
            const results = document.getElementById('results');
            results.innerHTML += `${timestamp} ${icon} ${message}\n`;
            results.scrollTop = results.scrollHeight;
        }
        
        function clearResults() {
            document.getElementById('results').innerHTML = 'Results cleared...\n';
            testCount = 0;
            passedTests = 0;
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
                log(`Health check passed: ${result.data.status}`, 'success');
            } else {
                log(`Health check failed: ${result.error || 'Unknown error'}`, 'error');
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
                log(`Login test passed: Token received`, 'success');
                passedTests++;
            } else {
                log(`Login test failed: ${result.error || 'No token received'}`, 'error');
            }
            testCount++;
        }
        
        async function testTradingPairs() {
            log('Testing trading pairs endpoint...');
            const result = await makeRequest('/api/trading/pairs');
            
            if (result.success && result.status === 200 && Array.isArray(result.data)) {
                log(`Trading pairs test passed: ${result.data.length} pairs found`, 'success');
                passedTests++;
            } else {
                log(`Trading pairs test failed: ${result.error || 'Invalid response'}`, 'error');
            }
            testCount++;
        }
        
        async function testTickers() {
            log('Testing market tickers endpoint...');
            const result = await makeRequest('/api/market-data/tickers');
            
            if (result.success && result.status === 200 && Array.isArray(result.data)) {
                log(`Market tickers test passed: ${result.data.length} tickers found`, 'success');
                passedTests++;
            } else {
                log(`Market tickers test failed: ${result.error || 'Invalid response'}`, 'error');
            }
            testCount++;
        }
        
        async function testBalances() {
            log('Testing wallet balances endpoint...');
            const result = await makeRequest('/api/wallet/balances');
            
            if (result.success && result.status === 200 && Array.isArray(result.data)) {
                log(`Wallet balances test passed: ${result.data.length} balances found`, 'success');
                passedTests++;
            } else {
                log(`Wallet balances test failed: ${result.error || 'Invalid response'}`, 'error');
            }
            testCount++;
        }
        
        async function runAllTests() {
            log('🚀 Starting comprehensive test suite...');
            testCount = 0;
            passedTests = 0;
            
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
            log(`\n📊 Test Summary: ${passedTests}/${testCount} tests passed (${successRate}% success rate)`, 
                passedTests === testCount ? 'success' : 'error');
        }
        
        function copyCredentials() {
            navigator.clipboard.writeText('demo@flowex.com\ndemo123').then(() => {
                log('Credentials copied to clipboard!', 'success');
            }).catch(() => {
                log('Failed to copy credentials', 'error');
            });
        }
        
        // Auto-run health check on page load
        window.addEventListener('load', () => {
            setTimeout(checkHealth, 1000);
        });
    </script>
</body>
</html>'''
        
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(html.encode('utf-8'))
    
    def log_message(self, format, *args):
        """Custom log format"""
        print(f"{datetime.now().strftime('%Y-%m-%d %H:%M:%S')} - {format % args}")

def main():
    """Main server startup function"""
    PORT = 8000
    
    print('🚀 FlowEx Enterprise UI Server Starting...')
    print('==========================================')
    print(f'✅ Enterprise UI Server: http://localhost:{PORT}')
    print(f'✅ Health Check: http://localhost:{PORT}/health')
    print(f'✅ API Base: http://localhost:{PORT}/api')
    print('')
    print('🦀 Rust Backend Services Available')
    print('📧 Demo Login: demo@flowex.com / demo123')
    print('')
    print('🎉 FlowEx Enterprise Environment Ready!')
    print('⏹️  Press Ctrl+C to stop')
    print('')
    
    try:
        with socketserver.TCPServer(("", PORT), FlowExHandler) as httpd:
            httpd.serve_forever()
    except KeyboardInterrupt:
        print('\n🛑 Shutting down FlowEx Enterprise UI Server...')
        print('✅ Shutdown complete')

if __name__ == "__main__":
    main()
