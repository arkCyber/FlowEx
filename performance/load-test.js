/**
 * FlowEx Production Load Testing Suite
 * ===================================
 * 
 * Comprehensive performance testing for FlowEx trading platform
 * Created by arkSong (arksong2018@gmail.com)
 * 
 * This test suite validates:
 * - API response times under load
 * - Trading system performance
 * - Database performance
 * - WebSocket connection handling
 * - System resource utilization
 */

import http from 'k6/http';
import ws from 'k6/ws';
import { check, sleep, group } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';
import { randomString, randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

// Custom metrics
const errorRate = new Rate('error_rate');
const responseTime = new Trend('response_time');
const tradingLatency = new Trend('trading_latency');
const wsConnectionTime = new Trend('ws_connection_time');
const orderProcessingTime = new Trend('order_processing_time');
const authenticationTime = new Trend('authentication_time');
const failedRequests = new Counter('failed_requests');
const successfulTrades = new Counter('successful_trades');

// Test configuration
export const options = {
  scenarios: {
    // Baseline load test
    baseline_load: {
      executor: 'constant-vus',
      vus: 50,
      duration: '5m',
      tags: { test_type: 'baseline' },
    },
    
    // Spike test
    spike_test: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '2m', target: 100 },
        { duration: '1m', target: 500 },  // Spike
        { duration: '2m', target: 100 },
        { duration: '1m', target: 0 },
      ],
      tags: { test_type: 'spike' },
    },
    
    // Stress test
    stress_test: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '5m', target: 100 },
        { duration: '10m', target: 200 },
        { duration: '10m', target: 300 },
        { duration: '5m', target: 0 },
      ],
      tags: { test_type: 'stress' },
    },
    
    // WebSocket test
    websocket_test: {
      executor: 'constant-vus',
      vus: 100,
      duration: '10m',
      exec: 'websocketTest',
      tags: { test_type: 'websocket' },
    },
    
    // Trading simulation
    trading_simulation: {
      executor: 'constant-arrival-rate',
      rate: 10, // 10 trades per second
      timeUnit: '1s',
      duration: '10m',
      preAllocatedVUs: 50,
      maxVUs: 200,
      exec: 'tradingSimulation',
      tags: { test_type: 'trading' },
    },
  },
  
  thresholds: {
    // Overall performance thresholds
    http_req_duration: ['p(95)<500', 'p(99)<1000'],
    http_req_failed: ['rate<0.01'], // Less than 1% failure rate
    
    // Custom metric thresholds
    error_rate: ['rate<0.05'],
    trading_latency: ['p(95)<200', 'p(99)<500'],
    ws_connection_time: ['p(95)<1000'],
    order_processing_time: ['p(95)<100'],
    authentication_time: ['p(95)<300'],
    
    // Scenario-specific thresholds
    'http_req_duration{test_type:baseline}': ['p(95)<300'],
    'http_req_duration{test_type:spike}': ['p(95)<1000'],
    'http_req_duration{test_type:stress}': ['p(95)<800'],
  },
};

// Test configuration
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8000';
const WS_URL = __ENV.WS_URL || 'ws://localhost:8003';

// Test data
const TRADING_PAIRS = ['BTCUSDT', 'ETHUSDT', 'ADAUSDT', 'DOTUSDT'];
const ORDER_TYPES = ['limit', 'market'];
const ORDER_SIDES = ['buy', 'sell'];

// Authentication helper
function authenticate() {
  const loginData = {
    email: `testuser${randomIntBetween(1, 1000)}@example.com`,
    password: 'TestPassword123!',
  };
  
  const startTime = Date.now();
  const response = http.post(`${BASE_URL}/api/auth/login`, JSON.stringify(loginData), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  const authTime = Date.now() - startTime;
  authenticationTime.add(authTime);
  
  if (response.status === 200) {
    const body = JSON.parse(response.body);
    return body.data.token;
  }
  
  return null;
}

// Main test scenario
export default function () {
  group('Authentication Flow', () => {
    const token = authenticate();
    
    check(token, {
      'authentication successful': (t) => t !== null,
    });
    
    if (!token) {
      failedRequests.add(1);
      return;
    }
    
    const headers = {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    };
    
    group('API Health Checks', () => {
      const services = [
        '/health',
        '/api/auth/health',
        '/api/trading/health',
        '/api/market-data/health',
        '/api/wallet/health',
      ];
      
      services.forEach(endpoint => {
        const response = http.get(`${BASE_URL}${endpoint}`);
        
        check(response, {
          [`${endpoint} is healthy`]: (r) => r.status === 200,
          [`${endpoint} response time OK`]: (r) => r.timings.duration < 100,
        });
        
        errorRate.add(response.status !== 200);
        responseTime.add(response.timings.duration);
      });
    });
    
    group('Market Data Operations', () => {
      // Get all trading pairs
      const pairsResponse = http.get(`${BASE_URL}/api/trading/pairs`, { headers });
      
      check(pairsResponse, {
        'trading pairs retrieved': (r) => r.status === 200,
        'trading pairs response time OK': (r) => r.timings.duration < 200,
      });
      
      // Get ticker data
      const tickerResponse = http.get(`${BASE_URL}/api/market-data/tickers`, { headers });
      
      check(tickerResponse, {
        'tickers retrieved': (r) => r.status === 200,
        'tickers response time OK': (r) => r.timings.duration < 300,
      });
      
      // Get specific ticker
      const symbol = TRADING_PAIRS[randomIntBetween(0, TRADING_PAIRS.length - 1)];
      const specificTickerResponse = http.get(`${BASE_URL}/api/market-data/ticker/${symbol}`, { headers });
      
      check(specificTickerResponse, {
        'specific ticker retrieved': (r) => r.status === 200,
        'specific ticker response time OK': (r) => r.timings.duration < 150,
      });
    });
    
    group('Wallet Operations', () => {
      // Get all balances
      const balancesResponse = http.get(`${BASE_URL}/api/wallet/balances`, { headers });
      
      check(balancesResponse, {
        'balances retrieved': (r) => r.status === 200,
        'balances response time OK': (r) => r.timings.duration < 200,
      });
      
      // Get transaction history
      const transactionsResponse = http.get(`${BASE_URL}/api/wallet/transactions`, { headers });
      
      check(transactionsResponse, {
        'transactions retrieved': (r) => r.status === 200,
        'transactions response time OK': (r) => r.timings.duration < 300,
      });
    });
    
    group('Trading Operations', () => {
      // Get active orders
      const ordersResponse = http.get(`${BASE_URL}/api/trading/orders`, { headers });
      
      check(ordersResponse, {
        'orders retrieved': (r) => r.status === 200,
        'orders response time OK': (r) => r.timings.duration < 200,
      });
      
      // Create a test order
      const orderData = {
        trading_pair: TRADING_PAIRS[randomIntBetween(0, TRADING_PAIRS.length - 1)],
        side: ORDER_SIDES[randomIntBetween(0, ORDER_SIDES.length - 1)],
        order_type: ORDER_TYPES[randomIntBetween(0, ORDER_TYPES.length - 1)],
        quantity: (Math.random() * 0.1 + 0.001).toFixed(6),
        price: ORDER_TYPES[0] === 'limit' ? (Math.random() * 1000 + 100).toFixed(2) : null,
      };
      
      const startTime = Date.now();
      const createOrderResponse = http.post(
        `${BASE_URL}/api/trading/orders`,
        JSON.stringify(orderData),
        { headers }
      );
      
      const orderTime = Date.now() - startTime;
      orderProcessingTime.add(orderTime);
      
      check(createOrderResponse, {
        'order created successfully': (r) => r.status === 201,
        'order creation time OK': (r) => r.timings.duration < 500,
      });
      
      if (createOrderResponse.status === 201) {
        successfulTrades.add(1);
      } else {
        failedRequests.add(1);
      }
    });
  });
  
  sleep(1);
}

// WebSocket test scenario
export function websocketTest() {
  const startTime = Date.now();
  
  const response = ws.connect(`${WS_URL}/ws`, {}, function (socket) {
    const connectionTime = Date.now() - startTime;
    wsConnectionTime.add(connectionTime);
    
    socket.on('open', () => {
      console.log('WebSocket connected');
      
      // Subscribe to market data
      socket.send(JSON.stringify({
        type: 'subscribe',
        channel: 'ticker',
        symbol: 'BTCUSDT'
      }));
      
      socket.send(JSON.stringify({
        type: 'subscribe',
        channel: 'orderbook',
        symbol: 'BTCUSDT'
      }));
    });
    
    socket.on('message', (data) => {
      const message = JSON.parse(data);
      
      check(message, {
        'valid message received': (m) => m.type !== undefined,
        'message has data': (m) => m.data !== undefined,
      });
    });
    
    socket.on('error', (e) => {
      console.log('WebSocket error:', e);
      failedRequests.add(1);
    });
    
    // Keep connection alive for test duration
    socket.setTimeout(() => {
      socket.close();
    }, 30000);
  });
  
  check(response, {
    'WebSocket connection established': (r) => r && r.status === 101,
  });
}

// Trading simulation scenario
export function tradingSimulation() {
  const token = authenticate();
  
  if (!token) {
    failedRequests.add(1);
    return;
  }
  
  const headers = {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
  
  // Simulate realistic trading behavior
  const tradingPair = TRADING_PAIRS[randomIntBetween(0, TRADING_PAIRS.length - 1)];
  
  // Get current market price
  const tickerResponse = http.get(`${BASE_URL}/api/market-data/ticker/${tradingPair}`, { headers });
  
  if (tickerResponse.status !== 200) {
    failedRequests.add(1);
    return;
  }
  
  const ticker = JSON.parse(tickerResponse.body).data;
  const currentPrice = parseFloat(ticker.price);
  
  // Create order with realistic price
  const side = ORDER_SIDES[randomIntBetween(0, ORDER_SIDES.length - 1)];
  const priceVariation = (Math.random() - 0.5) * 0.02; // ±1% price variation
  const orderPrice = (currentPrice * (1 + priceVariation)).toFixed(2);
  
  const orderData = {
    trading_pair: tradingPair,
    side: side,
    order_type: 'limit',
    quantity: (Math.random() * 0.1 + 0.001).toFixed(6),
    price: orderPrice,
  };
  
  const startTime = Date.now();
  const orderResponse = http.post(
    `${BASE_URL}/api/trading/orders`,
    JSON.stringify(orderData),
    { headers }
  );
  
  const tradingTime = Date.now() - startTime;
  tradingLatency.add(tradingTime);
  
  check(orderResponse, {
    'trading order successful': (r) => r.status === 201,
    'trading latency acceptable': () => tradingTime < 200,
  });
  
  if (orderResponse.status === 201) {
    successfulTrades.add(1);
  } else {
    failedRequests.add(1);
  }
}

// Setup function
export function setup() {
  console.log('Starting FlowEx performance tests...');
  console.log(`Base URL: ${BASE_URL}`);
  console.log(`WebSocket URL: ${WS_URL}`);
  
  // Verify system is ready
  const healthResponse = http.get(`${BASE_URL}/health`);
  
  if (healthResponse.status !== 200) {
    throw new Error('System health check failed - aborting tests');
  }
  
  console.log('System health check passed - proceeding with tests');
  
  return {
    startTime: Date.now(),
  };
}

// Teardown function
export function teardown(data) {
  const duration = (Date.now() - data.startTime) / 1000;
  console.log(`Performance tests completed in ${duration} seconds`);
  
  // Generate summary report
  console.log('=== Performance Test Summary ===');
  console.log(`Total test duration: ${duration}s`);
  console.log('Custom metrics will be available in the test results');
}
