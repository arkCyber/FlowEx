# Contributing to FlowEx

Thank you for your interest in contributing to FlowEx! We welcome contributions from the community and are excited to work with you to make FlowEx even better.

## 🌟 Ways to Contribute

### 🐛 Bug Reports
- Report bugs through [GitHub Issues](https://github.com/arkCyber/FlowEx/issues)
- Include detailed reproduction steps
- Provide system information and logs
- Use the bug report template

### 💡 Feature Requests
- Suggest new features through [GitHub Issues](https://github.com/arkCyber/FlowEx/issues)
- Describe the use case and expected behavior
- Include mockups or examples if applicable
- Use the feature request template

### 🔧 Code Contributions
- Fix bugs and implement new features
- Improve performance and security
- Add tests and documentation
- Follow our coding standards

### 📚 Documentation
- Improve existing documentation
- Add tutorials and examples
- Translate documentation
- Fix typos and clarify content

### 🧪 Testing
- Write additional tests
- Improve test coverage
- Report test failures
- Add performance benchmarks

## 🚀 Getting Started

### Prerequisites
- **Rust 1.75+** - [Install Rust](https://rustup.rs/)
- **Node.js 20+** - For frontend development
- **PostgreSQL 15+** - Database for persistent storage
- **Redis 7+** - Caching and session management
- **Docker & Docker Compose** - For containerized development
- **Kubernetes** (optional) - For production deployment testing

### 1. Fork the Repository
```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/FlowEx.git
cd FlowEx

# Add upstream remote
git remote add upstream https://github.com/arkCyber/FlowEx.git
```

### 2. Set Up Development Environment
```bash
# Install dependencies
rustup update
cargo --version

# Set up environment
cp .env.example .env

# Start infrastructure
docker-compose up -d postgres redis

# Run database migrations
./scripts/migrate.sh

# Run tests to ensure everything works
cargo test --workspace
```

### 3. Create a Feature Branch
```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

## 📝 How to Contribute

### Reporting Bugs
1. Check existing [Issues](https://github.com/arkCyber/FlowEx/issues)
2. Create new issue with:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details

### Code Contributions
1. Create feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make changes following coding standards
3. Add tests and ensure they pass:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. Commit with clear message:
   ```bash
   git commit -m "feat: add new trading feature"
   ```

5. Push and create Pull Request

## 🎯 Coding Standards

### Rust Code
- Follow [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` and `cargo clippy`
- Write comprehensive tests
- Document public APIs

### Commit Messages
Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `test:` for tests
- `refactor:` for refactoring
- `chore:` for maintenance

## 🧪 Testing
- Write unit tests for new functionality
- Ensure integration tests pass
- Add performance tests for critical paths
- Update documentation

## 📞 Getting Help
- [GitHub Discussions](https://github.com/arkCyber/FlowEx/discussions)
- Create issues for bugs/features
- Contact: arksong2018@gmail.com

Thank you for contributing! 🚀

---
**Created with ❤️ by arkSong and the FlowEx community**
