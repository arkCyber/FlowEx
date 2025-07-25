#!/bin/bash

# FlowEx测试覆盖率分析脚本
# =============================
# 
# 全面分析FlowEx项目的测试覆盖率，确保每个函数都有对应的测试
# Created by arkSong (arksong2018@gmail.com)

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKEND_DIR="$PROJECT_ROOT/backend"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

log_header() {
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${PURPLE}$*${NC}"
    echo -e "${PURPLE}========================================${NC}"
}

# 检查项目结构
check_project_structure() {
    log_header "检查项目结构"
    
    if [[ ! -d "$BACKEND_DIR" ]]; then
        log_error "Backend目录不存在: $BACKEND_DIR"
        exit 1
    fi
    
    log_success "项目结构检查通过"
    echo ""
}

# 分析测试覆盖率
analyze_test_coverage() {
    log_header "分析测试覆盖率"
    
    local total_files=0
    local files_with_tests=0
    local files_without_tests=0
    
    # 查找所有Rust源文件
    while IFS= read -r -d '' file; do
        ((total_files++))
        
        if grep -q "#\[cfg(test)\]" "$file"; then
            ((files_with_tests++))
            echo -e "${GREEN}✓${NC} $(basename "$file")"
        else
            ((files_without_tests++))
            echo -e "${RED}✗${NC} $(basename "$file")"
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)
    
    local coverage_percent=$((files_with_tests * 100 / total_files))
    
    echo ""
    log_info "测试覆盖率统计:"
    echo "  总文件数: $total_files"
    echo "  有测试的文件: $files_with_tests"
    echo "  缺少测试的文件: $files_without_tests"
    echo -e "  覆盖率: ${CYAN}${coverage_percent}%${NC}"
    
    if [[ $coverage_percent -ge 90 ]]; then
        log_success "测试覆盖率优秀 (≥90%)"
    elif [[ $coverage_percent -ge 70 ]]; then
        log_warning "测试覆盖率良好 (70-89%)"
    else
        log_error "测试覆盖率需要改进 (<70%)"
    fi
    
    echo ""
}

# 分析缺失测试的文件
analyze_missing_tests() {
    log_header "分析缺失测试的文件"
    
    local missing_files=()
    
    while IFS= read -r -d '' file; do
        if ! grep -q "#\[cfg(test)\]" "$file"; then
            missing_files+=("$file")
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)
    
    if [[ ${#missing_files[@]} -eq 0 ]]; then
        log_success "所有文件都有测试模块！"
        return 0
    fi
    
    log_warning "以下文件缺少测试模块:"
    
    # 按目录分组显示
    local services_files=()
    local shared_files=()
    local other_files=()
    
    for file in "${missing_files[@]}"; do
        if [[ "$file" == *"/services/"* ]]; then
            services_files+=("$file")
        elif [[ "$file" == *"/shared/"* ]]; then
            shared_files+=("$file")
        else
            other_files+=("$file")
        fi
    done
    
    if [[ ${#services_files[@]} -gt 0 ]]; then
        echo ""
        echo -e "${YELLOW}服务模块 (${#services_files[@]}个文件):${NC}"
        for file in "${services_files[@]}"; do
            local relative_path="${file#$PROJECT_ROOT/}"
            echo "  - $relative_path"
        done
    fi

    if [[ ${#shared_files[@]} -gt 0 ]]; then
        echo ""
        echo -e "${YELLOW}共享库模块 (${#shared_files[@]}个文件):${NC}"
        for file in "${shared_files[@]}"; do
            local relative_path="${file#$PROJECT_ROOT/}"
            echo "  - $relative_path"
        done
    fi

    if [[ ${#other_files[@]} -gt 0 ]]; then
        echo ""
        echo -e "${YELLOW}其他模块 (${#other_files[@]}个文件):${NC}"
        for file in "${other_files[@]}"; do
            local relative_path="${file#$PROJECT_ROOT/}"
            echo "  - $relative_path"
        done
    fi
    
    echo ""
}

# 分析测试质量
analyze_test_quality() {
    log_header "分析测试质量"
    
    local total_test_functions=0
    local detailed_tests=0
    local placeholder_tests=0
    
    while IFS= read -r -d '' file; do
        if grep -q "#\[cfg(test)\]" "$file"; then
            # 统计测试函数数量
            local test_count=0
            local tokio_test_count=0

            if grep -q "#\[test\]" "$file"; then
                test_count=$(grep -c "#\[test\]" "$file")
            fi

            if grep -q "#\[tokio::test\]" "$file"; then
                tokio_test_count=$(grep -c "#\[tokio::test\]" "$file")
            fi

            total_test_functions=$((total_test_functions + test_count + tokio_test_count))
            
            # 检查测试质量
            if grep -q "assert_eq!\|assert!\|assert_ne!" "$file"; then
                ((detailed_tests++))
            else
                ((placeholder_tests++))
            fi
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)
    
    echo "  总测试函数数: $total_test_functions"
    echo "  详细测试文件: $detailed_tests"
    echo "  占位测试文件: $placeholder_tests"
    
    if [[ $total_test_functions -gt 100 ]]; then
        log_success "测试函数数量充足"
    elif [[ $total_test_functions -gt 50 ]]; then
        log_warning "测试函数数量适中"
    else
        log_error "测试函数数量不足"
    fi
    
    echo ""
}

# 运行测试编译检查
run_test_compilation() {
    log_header "运行测试编译检查"
    
    cd "$PROJECT_ROOT"
    
    log_info "检查测试编译..."
    if cargo check --tests --workspace > /tmp/test_compilation.log 2>&1; then
        log_success "所有测试编译通过"
    else
        log_error "测试编译失败，详细信息:"
        cat /tmp/test_compilation.log | tail -20
        echo ""
        log_warning "请修复编译错误后重新运行"
    fi
    
    echo ""
}

# 生成测试报告
generate_test_report() {
    log_header "生成测试报告"
    
    local report_file="$PROJECT_ROOT/test-coverage-report.md"
    
    cat > "$report_file" << EOF
# FlowEx测试覆盖率报告

**生成时间**: $(date '+%Y-%m-%d %H:%M:%S')  
**项目**: FlowEx Enterprise Trading Platform  
**作者**: arkSong (arksong2018@gmail.com)  

## 📊 测试覆盖率统计

EOF
    
    # 重新计算统计数据
    local total_files=0
    local files_with_tests=0
    
    while IFS= read -r -d '' file; do
        ((total_files++))
        if grep -q "#\[cfg(test)\]" "$file"; then
            ((files_with_tests++))
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)
    
    local coverage_percent=$((files_with_tests * 100 / total_files))
    
    cat >> "$report_file" << EOF
- **总文件数**: $total_files
- **有测试的文件**: $files_with_tests
- **测试覆盖率**: $coverage_percent%

## 📋 详细分析

### ✅ 已完成测试的模块

EOF
    
    # 列出有测试的文件
    while IFS= read -r -d '' file; do
        if grep -q "#\[cfg(test)\]" "$file"; then
            local relative_path="${file#$PROJECT_ROOT/}"
            echo "- \`$relative_path\`" >> "$report_file"
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)

    cat >> "$report_file" << EOF

### ❌ 缺少测试的模块

EOF

    # 列出缺少测试的文件
    while IFS= read -r -d '' file; do
        if ! grep -q "#\[cfg(test)\]" "$file"; then
            local relative_path="${file#$PROJECT_ROOT/}"
            echo "- \`$relative_path\`" >> "$report_file"
        fi
    done < <(find "$BACKEND_DIR" -name "*.rs" -type f -print0)
    
    cat >> "$report_file" << EOF

## 🎯 改进建议

1. **优先级高**: 为核心服务模块添加测试
2. **优先级中**: 为共享库模块添加测试
3. **优先级低**: 为辅助模块添加测试

## 📈 测试质量标准

- ✅ 每个公共函数都应该有对应的测试
- ✅ 测试应该覆盖正常情况和边界情况
- ✅ 测试应该包含错误处理验证
- ✅ 测试应该有性能基准测试
- ✅ 测试应该验证并发安全性

---

**报告生成**: FlowEx测试覆盖率分析工具  
**维护者**: arkSong (arksong2018@gmail.com)
EOF
    
    log_success "测试报告已生成: $report_file"
    echo ""
}

# 提供改进建议
provide_improvement_suggestions() {
    log_header "改进建议"
    
    echo -e "${CYAN}为了达到生产标准，建议:${NC}"
    echo ""
    echo "1. 🎯 测试覆盖率目标: 95%以上"
    echo "2. 🧪 每个函数都应该有单元测试"
    echo "3. 🔄 添加集成测试验证服务间交互"
    echo "4. ⚡ 添加性能基准测试"
    echo "5. 🔒 添加安全性测试"
    echo "6. 🌐 添加并发安全性测试"
    echo "7. 📊 使用cargo-tarpaulin生成详细覆盖率报告"
    echo ""
    echo -e "${YELLOW}快速添加测试的命令:${NC}"
    echo "  cargo test --workspace"
    echo "  cargo test --workspace --release"
    echo "  cargo install cargo-tarpaulin"
    echo "  cargo tarpaulin --out Html"
    echo ""
}

# 主函数
main() {
    log_header "FlowEx测试覆盖率分析"
    echo "Created by arkSong (arksong2018@gmail.com)"
    echo ""
    
    check_project_structure
    analyze_test_coverage
    analyze_missing_tests
    analyze_test_quality
    run_test_compilation
    generate_test_report
    provide_improvement_suggestions
    
    log_success "测试覆盖率分析完成！"
}

# 运行主函数
main "$@"
