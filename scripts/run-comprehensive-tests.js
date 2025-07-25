#!/usr/bin/env node

// FlowEx Comprehensive Testing Suite
// Enterprise-grade testing with detailed reporting

const http = require('http');

console.log('🧪 FlowEx Comprehensive Testing Suite');
console.log('====================================');

const CONFIG = {
  BACKEND_URL: 'http://localhost:8000',
  FRONTEND_URL: 'http://localhost:3000',
  TIMEOUT: 5000
};

let testResults = {
  total: 0,
  passed: 0,
  failed: 0,
  tests: []
};

// Test helper function
async function runTest(name, testFn) {
  testResults.total++;
  const startTime = Date.now();
  
  try {
    await testFn();
    const duration = Date.now() - startTime;
    testResults.passed++;
    testResults.tests.push({ name, status: 'PASS', duration });
    console.log(`✅ ${name} (${duration}ms)`);
  } catch (error) {
    const duration = Date.now() - startTime;
    testResults.failed++;
    testResults.tests.push({ name, status: 'FAIL', duration, error: error.message });
    console.log(`❌ ${name} (${duration}ms) - ${error.message}`);
  }
}

// HTTP request helper
function makeRequest(url, options = {}) {
  return new Promise((resolve, reject) => {
    const req = http.request(url, options, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          const parsed = data ? JSON.parse(data) : {};
          resolve({ status: res.statusCode, data: parsed, headers: res.headers });
        } catch (e) {
          resolve({ status: res.statusCode, data: data, headers: res.headers });
        }
      });
    });
    
    req.on('error', reject);
    req.setTimeout(CONFIG.TIMEOUT, () => {
      req.destroy();
      reject(new Error('Request timeout'));
    });
    
    if (options.body) {
      req.write(options.body);
    }
    req.end();
  });
}

// Test suites
async function testHealthEndpoints() {
  console.log('\n🏥 Health Check Tests');
  console.log('====================');
  
  await runTest('Backend Health Check', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/health');
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
    if (!response.data.status) throw new Error('Missing status field');
    if (response.data.status !== 'healthy') throw new Error('Service not healthy');
  });
  
  await runTest('Frontend Availability', async () => {
    const response = await makeRequest(CONFIG.FRONTEND_URL);
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
  });
}

async function testAuthenticationAPI() {
  console.log('\n🔐 Authentication API Tests');
  console.log('===========================');
  
  await runTest('Valid Login', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: 'demo@flowex.com', password: 'demo123' })
    });
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
    if (!response.data.token) throw new Error('Missing token in response');
    if (!response.data.user) throw new Error('Missing user in response');
  });
  
  await runTest('Invalid Login', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: 'invalid@example.com', password: 'wrong' })
    });
    if (response.status !== 401) throw new Error(`Expected 401, got ${response.status}`);
  });
}

async function testTradingAPI() {
  console.log('\n📈 Trading API Tests');
  console.log('====================');
  
  await runTest('Get Trading Pairs', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/api/trading/pairs');
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
    if (!Array.isArray(response.data)) throw new Error('Expected array response');
    if (response.data.length === 0) throw new Error('No trading pairs returned');
  });
}

async function testMarketDataAPI() {
  console.log('\n📊 Market Data API Tests');
  console.log('========================');
  
  await runTest('Get Market Tickers', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/api/market-data/tickers');
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
    if (!Array.isArray(response.data)) throw new Error('Expected array response');
    if (response.data.length === 0) throw new Error('No tickers returned');
    
    // Validate ticker structure
    const ticker = response.data[0];
    if (!ticker.symbol) throw new Error('Missing symbol in ticker');
    if (!ticker.price) throw new Error('Missing price in ticker');
  });
}

async function testWalletAPI() {
  console.log('\n💰 Wallet API Tests');
  console.log('===================');
  
  await runTest('Get Wallet Balances', async () => {
    const response = await makeRequest(CONFIG.BACKEND_URL + '/api/wallet/balances');
    if (response.status !== 200) throw new Error(`Expected 200, got ${response.status}`);
    if (!Array.isArray(response.data)) throw new Error('Expected array response');
    if (response.data.length === 0) throw new Error('No balances returned');
    
    // Validate balance structure
    const balance = response.data[0];
    if (!balance.currency) throw new Error('Missing currency in balance');
    if (balance.available === undefined) throw new Error('Missing available amount');
  });
}

async function testPerformance() {
  console.log('\n⚡ Performance Tests');
  console.log('===================');
  
  await runTest('Response Time < 1000ms', async () => {
    const startTime = Date.now();
    await makeRequest(CONFIG.BACKEND_URL + '/health');
    const duration = Date.now() - startTime;
    if (duration > 1000) throw new Error(`Response time ${duration}ms exceeds 1000ms`);
  });
  
  await runTest('Concurrent Requests', async () => {
    const promises = Array.from({ length: 5 }, () => 
      makeRequest(CONFIG.BACKEND_URL + '/health')
    );
    const results = await Promise.all(promises);
    const failedRequests = results.filter(r => r.status !== 200);
    if (failedRequests.length > 0) {
      throw new Error(`${failedRequests.length} out of 5 concurrent requests failed`);
    }
  });
}

async function generateTestReport() {
  console.log('\n📋 Test Report');
  console.log('==============');
  
  const successRate = testResults.total > 0 ? 
    (testResults.passed / testResults.total * 100).toFixed(1) : 0;
  
  console.log(`Total Tests: ${testResults.total}`);
  console.log(`✅ Passed: ${testResults.passed}`);
  console.log(`❌ Failed: ${testResults.failed}`);
  console.log(`📊 Success Rate: ${successRate}%`);
  
  if (testResults.failed > 0) {
    console.log('\n❌ Failed Tests:');
    testResults.tests
      .filter(t => t.status === 'FAIL')
      .forEach(t => console.log(`   - ${t.name}: ${t.error}`));
  }
  
  // Generate JSON report
  const report = {
    timestamp: new Date().toISOString(),
    summary: {
      total: testResults.total,
      passed: testResults.passed,
      failed: testResults.failed,
      successRate: parseFloat(successRate)
    },
    tests: testResults.tests
  };
  
  require('fs').writeFileSync('test-report.json', JSON.stringify(report, null, 2));
  console.log('\n📄 Detailed report saved to: test-report.json');
}

// Main test execution
async function runAllTests() {
  console.log('Starting comprehensive test suite...\n');
  
  try {
    await testHealthEndpoints();
    await testAuthenticationAPI();
    await testTradingAPI();
    await testMarketDataAPI();
    await testWalletAPI();
    await testPerformance();
    
    await generateTestReport();
    
    console.log('\n🎉 Test suite completed!');
    
    if (testResults.failed === 0) {
      console.log('✅ All tests passed!');
      process.exit(0);
    } else {
      console.log('❌ Some tests failed. Check the report above.');
      process.exit(1);
    }
    
  } catch (error) {
    console.error('💥 Test suite failed:', error.message);
    process.exit(1);
  }
}

// Check if backend is running before starting tests
async function checkBackendAvailability() {
  try {
    await makeRequest(CONFIG.BACKEND_URL + '/health');
    console.log('✅ Backend is available, starting tests...');
    runAllTests();
  } catch (error) {
    console.log('❌ Backend is not available. Please start the backend first:');
    console.log('   npm run dev:backend');
    console.log('   or');
    console.log('   npm run dev');
    process.exit(1);
  }
}

checkBackendAvailability();
