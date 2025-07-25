# FlowEx Frontend

Enterprise-grade React frontend for the FlowEx cryptocurrency trading platform.

## 🚀 Features

- **Modern React 18** with TypeScript and Vite
- **Material-UI (MUI)** for enterprise-grade UI components
- **Redux Toolkit** for state management with persistence
- **React Router** for client-side routing
- **React Hook Form** with Yup validation
- **PWA Support** with service worker and offline capabilities
- **Comprehensive Testing** with Vitest and Testing Library
- **Enterprise Security** with authentication and authorization
- **Real-time Updates** via WebSocket integration
- **Responsive Design** with mobile-first approach
- **Dark/Light Theme** support
- **Performance Optimized** with code splitting and lazy loading

## 📁 Project Structure

```
frontend/
├── public/                 # Static assets
├── src/
│   ├── components/        # Reusable UI components
│   ├── pages/            # Page components
│   ├── hooks/            # Custom React hooks
│   ├── services/         # API services
│   ├── store/            # Redux store and slices
│   ├── theme/            # Material-UI theme configuration
│   ├── types/            # TypeScript type definitions
│   ├── utils/            # Utility functions
│   └── styles/           # Global styles
├── docker/               # Docker configuration
└── docs/                 # Documentation
```

## 🛠️ Development

### Prerequisites

- Node.js 18+ 
- npm 9+

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run tests
npm test

# Run linting
npm run lint

# Format code
npm run format
```

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# API Configuration
VITE_API_BASE_URL=http://localhost:8001
VITE_WS_BASE_URL=ws://localhost:8001

# Application Configuration
VITE_APP_VERSION=1.0.0
VITE_ENVIRONMENT=development

# Feature Flags
VITE_ENABLE_TRADING=true
VITE_ENABLE_ADVANCED_CHARTS=true
VITE_ENABLE_NOTIFICATIONS=true
VITE_ENABLE_2FA=true
```

## 🏗️ Architecture

### State Management

- **Redux Toolkit** for global state
- **React Query** for server state
- **Local Storage** persistence for auth and UI preferences
- **WebSocket** integration for real-time updates

### Routing

- **React Router v6** with lazy loading
- **Protected routes** with authentication
- **Role-based access control**
- **404 error handling**

### API Integration

- **Axios** HTTP client with interceptors
- **Automatic token refresh**
- **Error handling and retry logic**
- **Request/response logging**

### Testing Strategy

- **Unit tests** for components and utilities
- **Integration tests** for API services
- **E2E tests** for critical user flows
- **Visual regression tests** for UI components

## 🎨 UI/UX

### Design System

- **Material-UI** components with custom theme
- **Consistent spacing** and typography
- **Accessible** components (WCAG 2.1 AA)
- **Responsive** design for all screen sizes

### Theme Support

- **Light/Dark mode** toggle
- **Custom color palette** for trading
- **Consistent branding** across components
- **User preference** persistence

## 🔒 Security

### Authentication

- **JWT token** based authentication
- **Automatic token refresh**
- **Secure storage** of credentials
- **Session management**

### Authorization

- **Role-based access control**
- **Permission-based UI rendering**
- **Protected API endpoints**
- **Audit logging**

## 📱 PWA Features

- **Service Worker** for offline support
- **App manifest** for installation
- **Push notifications** (when enabled)
- **Background sync** for critical operations

## 🚀 Deployment

### Docker

```bash
# Build Docker image
docker build -t flowex-frontend .

# Run container
docker run -p 3000:80 flowex-frontend
```

### Production Build

```bash
# Build optimized production bundle
npm run build

# Preview production build
npm run preview
```

## 📊 Performance

### Optimization Features

- **Code splitting** by route and feature
- **Lazy loading** of components
- **Bundle analysis** and optimization
- **Image optimization** and compression
- **Caching strategies** for static assets

### Monitoring

- **Performance metrics** tracking
- **Error boundary** for graceful failures
- **User analytics** (when enabled)
- **Real-time monitoring** integration

## 🧪 Testing

```bash
# Run all tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch

# Run E2E tests
npm run test:e2e
```

## 📚 Documentation

- [Component Library](./docs/components.md)
- [API Integration](./docs/api.md)
- [State Management](./docs/state.md)
- [Testing Guide](./docs/testing.md)
- [Deployment Guide](./docs/deployment.md)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For support and questions:

- Create an issue on GitHub
- Check the documentation
- Contact the development team

---

Built with ❤️ by the FlowEx Team
