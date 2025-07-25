# FlowEx 编程规范与标准

## 项目概述
FlowEx是一个企业级加密货币交易平台，采用现代化技术栈构建，注重性能、安全性和可维护性。

## 技术栈

### 前端技术栈
- **语言**: TypeScript 5.0+
- **框架**: React 18+ with Hooks
- **构建工具**: Vite 4+
- **包管理器**: pnpm 8+
- **UI框架**: Tailwind CSS 3+
- **状态管理**: Redux Toolkit + RTK Query
- **路由**: React Router v6
- **表单**: React Hook Form + Yup
- **图标**: Lucide React
- **测试**: Vitest + Testing Library
- **类型检查**: TypeScript strict mode

### 后端技术栈
- **语言**: Rust 1.70+
- **架构**: 微服务架构
- **框架**: Axum + Tokio
- **数据库**: PostgreSQL + SQLx
- **缓存**: Redis
- **消息队列**: RabbitMQ
- **容器化**: Docker + Kubernetes- **日志**: tracing + serde_json

## 前端编程规范

### 1. TypeScript 规范

#### 类型定义
```typescript
// ✅ 正确：使用interface定义对象类型
interface User {
  id: string
  email: string
  firstName: string
  lastName: string
  createdAt: Date
  updatedAt: Date
}

// ✅ 正确：使用type定义联合类型
type Theme = 'light' | 'dark'
type Status = 'loading' | 'success' | 'error'

// ✅ 正确：使用泛型
interface ApiResponse<T> {
  data: T
  message: string
  success: boolean
}
```

#### 命名规范
```typescript
// ✅ 正确：PascalCase用于类型、接口、组件
interface UserProfile {}
type OrderStatus = 'pending' | 'completed'
const UserCard: React.FC = () => {}

// ✅ 正确：camelCase用于变量、函数
const userName = 'john'
const fetchUserData = async () => {}

// ✅ 正确：SCREAMING_SNAKE_CASE用于常量
const API_BASE_URL = 'https://api.flowex.com'
const MAX_RETRY_ATTEMPTS = 3
```

### 2. React 组件规范

#### 函数组件结构
```typescript
/**
 * 用户资料卡片组件
 * 显示用户基本信息和操作按钮
 */
interface UserCardProps {
  user: User
  onEdit?: (user: User) => void
  className?: string
}

export const UserCard: React.FC<UserCardProps> = ({ 
  user, 
  onEdit, 
  className 
}) => {
  // 1. Hooks (useState, useEffect, custom hooks)
  const [isLoading, setIsLoading] = useState(false)
  const { theme } = useTheme()
  
  // 2. Event handlers
  const handleEdit = useCallback(() => {
    if (onEdit) {
      onEdit(user)
    }
  }, [user, onEdit])
  
  // 3. Effects
  useEffect(() => {
    // 组件挂载时的逻辑
  }, [])
  
  // 4. Early returns
  if (!user) {
    return <div>No user data</div>
  }
  
  // 5. Render
  return (
    <div className={cn('card', className)}>
      <h3 className="text-lg font-semibold text-white">
        {user.firstName} {user.lastName}
      </h3>
      <p className="text-stone-400">{user.email}</p>
      {onEdit && (
        <button 
          onClick={handleEdit}
          className="btn-primary mt-4"
          disabled={isLoading}
        >
          Edit Profile
        </button>
      )}
    </div>
  )
}
```

### 3. Tailwind CSS 规范

#### 暗亮模式设计
```typescript
// ✅ 正确：使用暗模式优先的设计
const ThemeCard = () => (
  <div className="
    bg-stone-900 dark:bg-stone-900 
    light:bg-white 
    text-white light:text-gray-900
    border border-stone-700 light:border-gray-200
    rounded-xl p-6 shadow-lg
  ">
    <h2 className="text-xl font-bold mb-4">Card Title</h2>
    <p className="text-stone-400 light:text-gray-600">
      Card content goes here
    </p>
  </div>
)

// ✅ 正确：暖色调配色方案
const WarmColors = {
  background: {
    primary: 'bg-stone-900',      // 主背景 - 深暖灰
    secondary: 'bg-stone-800',    // 次要背景 - 中暖灰  
    tertiary: 'bg-stone-700',     // 第三背景 - 浅暖灰
  },
  text: {
    primary: 'text-white',        // 主文本 - 白色
    secondary: 'text-stone-300',  // 次要文本 - 浅灰
    muted: 'text-stone-400',      // 弱化文本 - 中灰
  },
  accent: {
    warm: 'text-amber-400',       // 暖色强调 - 琥珀色
    primary: 'text-blue-400',     // 主色强调 - 蓝色
    success: 'text-green-400',    // 成功色 - 绿色
    error: 'text-red-400',        // 错误色 - 红色
  }
}
```

#### 组件样式规范
```typescript
// ✅ 正确：使用工具类组合
const buttonVariants = {
  primary: 'bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors duration-200',
  secondary: 'bg-stone-700 hover:bg-stone-600 text-white px-4 py-2 rounded-lg font-medium transition-colors duration-200',
  outline: 'border border-stone-600 hover:bg-stone-700 text-stone-300 hover:text-white px-4 py-2 rounded-lg font-medium transition-all duration-200'
}

// ✅ 正确：响应式设计
const ResponsiveGrid = () => (
  <div className="
    grid 
    grid-cols-1 
    md:grid-cols-2 
    lg:grid-cols-3 
    xl:grid-cols-4 
    gap-4 
    md:gap-6
  ">
    {/* Grid items */}
  </div>
)
```

### 4. 状态管理规范

#### Redux Toolkit Slice
```typescript
interface TradingState {
  orders: Order[]
  loading: boolean
  error: string | null
}

const initialState: TradingState = {
  orders: [],
  loading: false,
  error: null,
}

export const tradingSlice = createSlice({
  name: 'trading',
  initialState,
  reducers: {
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload
    },
    addOrder: (state, action: PayloadAction<Order>) => {
      state.orders.unshift(action.payload)
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchOrders.pending, (state) => {
        state.loading = true
        state.error = null
      })
      .addCase(fetchOrders.fulfilled, (state, action) => {
        state.loading = false
        state.orders = action.payload
      })
      .addCase(fetchOrders.rejected, (state, action) => {
        state.loading = false
        state.error = action.error.message || 'Failed to fetch orders'
      })
  },
})
```

### 5. 错误处理规范

```typescript
// ✅ 正确：统一错误处理
const useApiCall = <T>(apiCall: () => Promise<T>) => {
  const [data, setData] = useState<T | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  
  const execute = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)
      const result = await apiCall()
      setData(result)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error'
      setError(errorMessage)
      console.error('API call failed:', err)
    } finally {
      setLoading(false)
    }
  }, [apiCall])
  
  return { data, loading, error, execute }
}
```

### 6. 测试规范

```typescript
// ✅ 正确：组件测试
describe('UserCard', () => {
  const mockUser: User = {
    id: '1',
    email: 'test@example.com',
    firstName: 'John',
    lastName: 'Doe',
    createdAt: new Date(),
    updatedAt: new Date(),
  }
  
  it('renders user information correctly', () => {
    render(<UserCard user={mockUser} />)
    
    expect(screen.getByText('John Doe')).toBeInTheDocument()
    expect(screen.getByText('test@example.com')).toBeInTheDocument()
  })
  
  it('calls onEdit when edit button is clicked', () => {
    const mockOnEdit = vi.fn()
    render(<UserCard user={mockUser} onEdit={mockOnEdit} />)
    
    fireEvent.click(screen.getByText('Edit Profile'))
    expect(mockOnEdit).toHaveBeenCalledWith(mockUser)
  })
})
```

## 后端编程规范 (Rust)

### 1. 项目结构
```
crates/
├── flowex-core/          # 核心业务逻辑
├── flowex-auth/          # 认证服务
├── flowex-trading/       # 交易服务
├── flowex-market-data/   # 市场数据服务
├── flowex-wallet/        # 钱包服务
├── flowex-notification/  # 通知服务
└── flowex-gateway/       # API网关
```

### 2. 函数规范

```rust
use tracing::{info, error, warn, debug};
use anyhow::{Result, Context};

/// 创建新的交易订单
/// 
/// # Arguments
/// * `order_request` - 订单请求数据
/// * `user_id` - 用户ID
/// 
/// # Returns
/// * `Result<Order>` - 创建的订单或错误
/// 
/// # Errors
/// * 当用户余额不足时返回错误
/// * 当交易对不存在时返回错误
#[tracing::instrument(skip(db_pool))]
pub async fn create_order(
    order_request: CreateOrderRequest,
    user_id: Uuid,
    db_pool: &PgPool,
) -> Result<Order> {
    info!(
        user_id = %user_id,
        symbol = %order_request.symbol,
        order_type = %order_request.order_type,
        "Creating new order"
    );
    
    // 1. 验证输入参数
    validate_order_request(&order_request)
        .context("Invalid order request")?;
    
    // 2. 检查用户余额
    let balance = get_user_balance(user_id, &order_request.base_currency, db_pool)
        .await
        .context("Failed to get user balance")?;
    
    if balance < order_request.quantity {
        warn!(
            user_id = %user_id,
            required = %order_request.quantity,
            available = %balance,
            "Insufficient balance for order"
        );
        return Err(anyhow::anyhow!("Insufficient balance"));
    }
    
    // 3. 创建订单
    let order = Order {
        id: Uuid::new_v4(),
        user_id,
        symbol: order_request.symbol.clone(),
        order_type: order_request.order_type,
        quantity: order_request.quantity,
        price: order_request.price,
        status: OrderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // 4. 保存到数据库
    let saved_order = save_order(&order, db_pool)
        .await
        .context("Failed to save order to database")?;
    
    info!(
        order_id = %saved_order.id,
        user_id = %user_id,
        "Order created successfully"
    );
    
    Ok(saved_order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    
    #[tokio::test]
    async fn test_create_order_success() {
        // 测试成功创建订单
        let db_pool = setup_test_db().await;
        let user_id = Uuid::new_v4();
        
        // 设置测试数据
        setup_user_balance(&db_pool, user_id, "BTC", 1.0).await;
        
        let order_request = CreateOrderRequest {
            symbol: "BTC/USDT".to_string(),
            order_type: OrderType::Market,
            quantity: 0.5,
            price: None,
            base_currency: "BTC".to_string(),
        };
        
        // 执行测试
        let result = create_order(order_request, user_id, &db_pool).await;
        
        // 验证结果
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.user_id, user_id);
        assert_eq!(order.quantity, 0.5);
    }
    
    #[tokio::test]
    async fn test_create_order_insufficient_balance() {
        // 测试余额不足的情况
        let db_pool = setup_test_db().await;
        let user_id = Uuid::new_v4();
        
        // 设置测试数据 - 余额不足
        setup_user_balance(&db_pool, user_id, "BTC", 0.1).await;
        
        let order_request = CreateOrderRequest {
            symbol: "BTC/USDT".to_string(),
            order_type: OrderType::Market,
            quantity: 0.5, // 大于可用余额
            price: None,
            base_currency: "BTC".to_string(),
        };
        
        // 执行测试
        let result = create_order(order_request, user_id, &db_pool).await;
        
        // 验证结果
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Insufficient balance"));
    }
}
```

### 3. 错误处理规范

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },
    
    #[error("Invalid trading pair: {symbol}")]
    InvalidTradingPair { symbol: String },
    
    #[error("Order not found: {order_id}")]
    OrderNotFound { order_id: Uuid },
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

// 使用示例
pub async fn cancel_order(order_id: Uuid, user_id: Uuid) -> Result<(), TradingError> {
    let order = find_order_by_id(order_id)
        .await?
        .ok_or(TradingError::OrderNotFound { order_id })?;
    
    if order.user_id != user_id {
        return Err(TradingError::Validation(
            "User not authorized to cancel this order".to_string()
        ));
    }
    
    // 取消订单逻辑...
    Ok(())
}
```

### 4. 日志规范

```rust
use tracing::{info, warn, error, debug, trace};

// 在函数开始处记录关键信息
#[tracing::instrument(skip(db_pool))]
pub async fn process_payment(payment_id: Uuid, db_pool: &PgPool) -> Result<()> {
    info!(payment_id = %payment_id, "Starting payment processing");
    
    // 记录重要的业务逻辑步骤
    debug!("Validating payment data");
    
    // 记录警告
    if payment.amount > 10000.0 {
        warn!(
            payment_id = %payment_id,
            amount = %payment.amount,
            "Large payment amount detected"
        );
    }
    
    // 记录错误
    if let Err(e) = validate_payment(&payment) {
        error!(
            payment_id = %payment_id,
            error = %e,
            "Payment validation failed"
        );
        return Err(e);
    }
    
    info!(payment_id = %payment_id, "Payment processed successfully");
    Ok(())
}
```

## 包管理规范

### 使用 pnpm 命令

```bash
# 安装依赖
pnpm install

# 添加依赖
pnpm add package-name
pnpm add -D package-name  # 开发依赖

# 运行脚本
pnpm dev                  # 开发服务器
pnpm build               # 构建
pnpm test                # 测试
pnpm lint                # 代码检查
pnpm format              # 代码格式化

# 工作空间命令
pnpm --filter frontend dev
pnpm --filter backend test
```

## 提交规范

### Git Commit 格式
```
type(scope): description

feat(auth): add two-factor authentication
fix(trading): resolve order calculation bug
docs(readme): update installation instructions
style(ui): improve button hover effects
refactor(api): simplify error handling
test(trading): add unit tests for order service
chore(deps): update dependencies
```

## 代码审查清单

### 前端审查要点
- [ ] TypeScript类型定义完整
- [ ] 组件props有正确的类型
- [ ] 使用了适当的React hooks
- [ ] 错误边界处理
- [ ] 无障碍性(a11y)考虑
- [ ] 响应式设计
- [ ] 暗亮模式支持
- [ ] 性能优化(memo, callback)
- [ ] 测试覆盖率

### 后端审查要点
- [ ] 函数有完整的文档注释
- [ ] 错误处理完善
- [ ] 日志记录充分
- [ ] 单元测试覆盖
- [ ] 集成测试覆盖
- [ ] 数据库事务处理
- [ ] 安全性考虑
- [ ] 性能优化
- [ ] 内存安全

## 性能优化指南

### 前端性能
- 使用React.memo优化组件渲染
- 使用useMemo和useCallback优化计算
- 代码分割和懒加载
- 图片优化和懒加载
- Bundle分析和优化

### 后端性能
- 数据库查询优化
- 连接池配置
- 缓存策略
- 异步处理
- 内存管理

all comments in English!

这些规范将确保FlowEx项目的代码质量、可维护性和性能。
