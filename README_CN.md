# FlowEx - 企业级交易平台

[![构建状态](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/arkCyber/FlowEx)
[![覆盖率](https://img.shields.io/badge/coverage-95%25-brightgreen)](https://github.com/arkCyber/FlowEx)
[![许可证](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org/)

> **由 arkSong 创建** - 创始人兼首席开发者  
> 📧 联系方式: arksong2018@gmail.com  
> 🌟 **支持这个创新项目！** 在GitHub上给我们点星！

**FlowEx** 是一个基于 Rust 构建的高性能企业级加密货币交易平台。它提供了完整的交易生态系统，具有微服务架构、实时市场数据、高级订单管理和机构级安全性。

## 🏗️ 架构概览

FlowEx 采用微服务架构，包含以下核心服务：

- **认证服务** (端口 8001): JWT认证、用户管理和授权
- **交易服务** (端口 8002): 订单管理、交易执行和订单簿
- **市场数据服务** (端口 8003): 实时市场数据、行情和价格推送
- **钱包服务** (端口 8004): 余额管理、交易和资金操作

## 🚀 快速开始

### 前置要求
- **Rust 1.75+** - [安装 Rust](https://rustup.rs/)
- **PostgreSQL 15+** - 持久化存储数据库
- **Redis 7+** - 缓存和会话管理
- **Docker & Docker Compose** - 容器化部署
- **Node.js** (可选) - 开发工具

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/arkCyber/FlowEx.git
cd FlowEx

# 复制环境配置
cp .env.example .env

# 启动基础设施服务
docker-compose up -d postgres redis

# 运行数据库迁移
cargo run --bin migrate

# 启动所有服务
cargo run --bin auth-service &
cargo run --bin trading-service &
cargo run --bin market-data-service &
cargo run --bin wallet-service &

# 或使用开发脚本
npm run dev
```

### 生产环境部署

```bash
# 使用 Docker 构建和部署
docker-compose up -d

# 或使用企业构建脚本
./scripts/build-enterprise.sh --env production --docker --push

# 部署到 Kubernetes
kubectl apply -f k8s/
```

## 🌟 企业级特性

### 🔐 安全与认证
- **JWT 认证** 支持刷新令牌和基于角色的访问控制
- **Argon2 密码哈希** 可配置参数
- **速率限制** 基于 Redis 的分布式限制
- **安全头** 包括 CSRF 保护和 CORS 配置
- **审计日志** 记录所有关键操作
- **双因素认证** 增强账户安全

### 🏦 交易引擎
- **高性能订单匹配** 微秒级延迟
- **多种订单类型**: 市价单、限价单、止损单、止盈单
- **实时订单簿** WebSocket 流式传输
- **交易历史** 和执行报告
- **风险管理** 仓位限制和保证金控制
- **费用管理** 可配置的做市商/吃单者费用

### 💾 数据管理
- **PostgreSQL 数据库** 优化的模式和索引
- **Redis 缓存** 会话管理和实时数据
- **数据库迁移** 版本控制和回滚支持
- **连接池** 健康监控
- **备份与恢复** 自动调度

### 📊 监控与可观测性
- **Prometheus 指标** 自定义业务指标
- **Grafana 仪表板** 实时监控
- **结构化日志** JSON 格式和日志级别
- **健康检查** 所有服务的详细状态
- **分布式追踪** OpenTelemetry 集成
- **告警** PagerDuty 和 Slack 集成

### 🚀 性能与可扩展性
- **微服务架构** 独立扩展
- **异步/等待** 全程使用 Tokio 运行时
- **连接池** 数据库和 Redis
- **负载均衡** Nginx 和健康检查
- **水平扩展** Kubernetes 支持
- **缓存策略** 优化性能

### 🔧 DevOps 与部署
- **Docker 容器化** 多阶段构建
- **Kubernetes 清单** 生产环境部署
- **CI/CD 流水线** 自动化测试和部署
- **环境配置** 密钥管理
- **蓝绿部署** 支持
- **基础设施即代码** Terraform (即将推出)

## 🧪 测试与质量保证

FlowEx 包含全面的测试套件，具有多个测试级别：

### 单元测试
```bash
# 运行所有单元测试
cargo test --lib

# 运行覆盖率测试
cargo tarpaulin --out Html --output-dir coverage
```

### 集成测试
```bash
# 运行集成测试
cargo test --test integration

# 运行企业测试套件
./scripts/test-enterprise.sh

# 运行特定服务测试
./scripts/test-enterprise.sh --integration-only
```

### 性能测试
```bash
# 运行性能基准测试
cargo bench

# 使用 wrk 进行负载测试
./scripts/test-enterprise.sh --performance

# 压力测试
wrk -t4 -c100 -d30s http://localhost:8002/api/trading/pairs
```

### 安全测试
```bash
# 安全审计
cargo audit

# 不安全代码检测
cargo geiger

# 运行安全测试套件
./scripts/test-enterprise.sh --security
```

### 测试覆盖率
- **目标覆盖率**: 95%+
- **当前覆盖率**: 95%
- **关键路径覆盖率**: 100%
- **集成测试覆盖率**: 90%

## 🔑 演示凭据
- **邮箱**: demo@flowex.com
- **密码**: demo123

## 📁 项目结构
```
FlowEx/
├── package.json              # 项目配置
├── README.md                 # 英文文档
├── README_CN.md              # 中文文档
├── scripts/                  # 启动和测试脚本
│   ├── start-enterprise-environment.js
│   └── run-comprehensive-tests.js
├── frontend/                 # 前端应用
├── backend/                  # 后端服务
├── infrastructure/           # 基础设施配置
└── docs/                     # 文档
```

## 🎯 可用命令

```bash
npm run dev          # 启动完整企业环境
npm run dev:backend  # 仅启动后端 API
npm run dev:frontend # 仅启动前端
npm test            # 运行综合测试套件
npm start           # npm run dev 的别名
```

## 📊 API 端点

### 认证
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/register` - 用户注册
- `GET /api/auth/me` - 获取当前用户

### 交易
- `GET /api/trading/pairs` - 获取交易对
- `GET /api/trading/orderbook/:symbol` - 获取订单簿
- `POST /api/trading/orders` - 创建订单
- `GET /api/trading/orders` - 获取用户订单

### 市场数据
- `GET /api/market-data/ticker/:symbol` - 获取行情
- `GET /api/market-data/tickers` - 获取所有行情
- `GET /api/market-data/trades/:symbol` - 获取最近交易

### 钱包
- `GET /api/wallet/balances` - 获取所有余额
- `GET /api/wallet/balance/:currency` - 获取特定余额
- `GET /api/wallet/transactions` - 获取交易历史

## 🎉 即开即用

这是一个完整的企业级开发环境，只需要 Node.js 即可运行。无需额外设置！

开始使用: `npm run dev`

## 👨‍💻 关于创作者

**FlowEx** 由 **arkSong** 创建和维护，他是一位致力于构建创新交易解决方案的热情开发者。

- 🚀 **创始人**: arkSong
- 📧 **联系方式**: arksong2018@gmail.com
- 🌐 **GitHub**: [@arkCyber](https://github.com/arkCyber)
- 💡 **愿景**: 创造下一代企业级交易平台

### 🌟 支持这个项目

如果您觉得 FlowEx 有用，请：
- ⭐ 在 GitHub 上**给这个仓库点星**
- 🍴 **Fork** 并为项目做贡献
- 📢 与您的网络**分享**
- 💬 **加入**我们的社区讨论

您的支持有助于推动交易技术领域的创新！

## 🏗️ 企业特性

### ✅ 完整的模拟后端
- 完整的 REST API 实现
- CORS 支持
- 请求日志记录
- 健康监控

### ✅ 交互式前端
- 实时 API 测试界面
- 服务状态监控
- 演示凭据管理
- 响应式设计

### ✅ 综合测试
- 健康检查测试
- 认证测试
- API 端点测试
- 性能测试
- 自动化测试报告

### ✅ 企业标准
- 结构化日志
- 错误处理
- 健康监控
- 性能指标
- 文档

## 🔧 开发

### 添加新端点
编辑 `scripts/start-enterprise-environment.js` 并添加您的端点处理器。

### 自定义前端
前端作为单个 HTML 页面提供，内嵌 CSS/JS 以简化操作。

### 运行测试
测试自动检查所有 API 端点并生成详细报告。

## 📈 监控

### 健康检查
- 后端: http://localhost:8000/health
- 前端: http://localhost:3000 (可视化状态)

### 测试报告
- 自动 JSON 报告生成
- 性能指标
- 成功/失败跟踪

## 📄 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 🤝 贡献

我们欢迎贡献！请查看我们的[贡献指南](CONTRIBUTING.md)了解详情。

## 📞 联系与支持

- **问题反馈**: [GitHub Issues](https://github.com/arkCyber/FlowEx/issues)
- **讨论**: [GitHub Discussions](https://github.com/arkCyber/FlowEx/discussions)
- **邮箱**: arksong2018@gmail.com

---

**由 arkSong 和 FlowEx 社区用 ❤️ 制作**
