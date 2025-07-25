#!/usr/bin/env python3
import urllib.request
import json
import time

def test_api(name, url, method='GET', data=None, expected_status=200):
    try:
        if data:
            data = json.dumps(data).encode('utf-8')
            req = urllib.request.Request(url, data=data, headers={'Content-Type': 'application/json'})
            req.get_method = lambda: method
        else:
            req = urllib.request.Request(url)
        
        start_time = time.time()
        response = urllib.request.urlopen(req)
        response_time = int((time.time() - start_time) * 1000)
        
        if response.status == expected_status:
            result = json.loads(response.read().decode('utf-8'))
            print(f"✅ {name} - {response.status} ({response_time}ms)")
            return True, result
        else:
            print(f"❌ {name} - Expected {expected_status}, got {response.status}")
            return False, None
    except Exception as e:
        print(f"❌ {name} - Error: {str(e)}")
        return False, None

print("🧪 FlowEx Enterprise System Testing")
print("===================================")
print()

# Test results tracking
total_tests = 0
passed_tests = 0

# Health Check Test
print("🏥 Health Check Tests")
print("====================")
success, data = test_api("Backend Health Check", "http://localhost:8000/health")
total_tests += 1
if success: passed_tests += 1
print()

# Authentication Tests
print("🔐 Authentication Tests")
print("=======================")
success, data = test_api("Valid Login", "http://localhost:8000/api/auth/login", "POST", 
                        {"email": "demo@flowex.com", "password": "demo123"})
total_tests += 1
if success: 
    passed_tests += 1
    if data and 'token' in data:
        print(f"   Token received: {data['token'][:20]}...")

success, data = test_api("Invalid Login", "http://localhost:8000/api/auth/login", "POST", 
                        {"email": "invalid@example.com", "password": "wrong"}, 401)
total_tests += 1
if success: passed_tests += 1
print()

# Trading API Tests
print("📈 Trading API Tests")
print("===================")
success, data = test_api("Get Trading Pairs", "http://localhost:8000/api/trading/pairs")
total_tests += 1
if success: 
    passed_tests += 1
    if data:
        print(f"   Found {len(data)} trading pairs")
print()

# Market Data Tests
print("📊 Market Data Tests")
print("===================")
success, data = test_api("Get Market Tickers", "http://localhost:8000/api/market-data/tickers")
total_tests += 1
if success: 
    passed_tests += 1
    if data:
        print(f"   Found {len(data)} market tickers")
        for ticker in data[:2]:  # Show first 2 tickers
            print(f"   {ticker['symbol']}: ${ticker['price']} ({ticker['change']}%)")
print()

# Wallet API Tests
print("💰 Wallet API Tests")
print("==================")
success, data = test_api("Get Wallet Balances", "http://localhost:8000/api/wallet/balances")
total_tests += 1
if success: 
    passed_tests += 1
    if data:
        print(f"   Found {len(data)} wallet balances")
        for balance in data:
            print(f"   {balance['currency']}: {balance['available']} (available)")
print()

# Frontend Test
print("🌐 Frontend Tests")
print("================")
success, data = test_api("Frontend Interface", "http://localhost:8000/", expected_status=200)
total_tests += 1
if success: passed_tests += 1
print()

# Performance Test
print("⚡ Performance Tests")
print("===================")
start_time = time.time()
concurrent_requests = []
for i in range(5):
    success, _ = test_api(f"Concurrent Request {i+1}", "http://localhost:8000/health")
    concurrent_requests.append(success)

concurrent_success = sum(concurrent_requests)
total_time = int((time.time() - start_time) * 1000)
print(f"   Concurrent requests: {concurrent_success}/5 successful in {total_time}ms")
total_tests += 1
if concurrent_success == 5: passed_tests += 1
print()

# Final Report
print("📋 Test Summary")
print("==============")
success_rate = (passed_tests / total_tests * 100) if total_tests > 0 else 0
print(f"Total Tests: {total_tests}")
print(f"✅ Passed: {passed_tests}")
print(f"❌ Failed: {total_tests - passed_tests}")
print(f"📊 Success Rate: {success_rate:.1f}%")
print()

if passed_tests == total_tests:
    print("🎉 All tests passed! FlowEx Enterprise environment is working perfectly!")
else:
    print("⚠️  Some tests failed. Please check the results above.")

# Generate JSON report
report = {
    "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    "environment": "FlowEx Enterprise Development",
    "summary": {
        "total_tests": total_tests,
        "passed_tests": passed_tests,
        "failed_tests": total_tests - passed_tests,
        "success_rate": f"{success_rate:.1f}%"
    }
}

with open('system_test_report.json', 'w') as f:
    json.dump(report, f, indent=2)

print(f"\n📄 Detailed report saved to: system_test_report.json")
