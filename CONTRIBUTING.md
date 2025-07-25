# Contributing to FlowEx

Thank you for your interest in contributing to FlowEx! We welcome contributions from the community.

## 🚀 Getting Started

### Prerequisites
- Rust 1.75+
- Node.js 18+
- PostgreSQL 15+
- Redis 7+
- Docker and Docker Compose

### Development Setup
1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/FlowEx.git
   cd FlowEx
   ```

3. Set up environment:
   ```bash
   cp .env.example .env
   docker-compose up -d postgres redis
   cargo build
   npm install
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
