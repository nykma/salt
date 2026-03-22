# Salt - 密码哈希 CLI 工具

## 产品概述

**Salt** 是一个用 Rust 编写的高性能命令行工具，专门用于密码哈希的生成和验证。它提供简洁易用的 CLI 接口，支持业界主流的多种密码哈希算法，帮助开发者和系统管理员快速进行密码安全处理。

## 核心功能

### 1. 密码哈希生成
- 使用指定的哈希算法生成密码的安全哈希值
- 自动处理盐值（salt）的生成和管理
- 支持自定义盐值参数
- 输出格式清晰，易于集成到其他系统

### 2. 密码验证
- 验证给定的明文密码是否与已有的哈希值匹配
- 支持多种哈希算法的自动识别和验证
- 返回清晰的验证结果（匹配/不匹配）

### 3. 算法支持
支持十余种主流密码哈希算法，包括但不限于：
- **PBKDF2** - 基于密钥推导函数的标准算法
- **bcrypt** - 自适应哈希算法，具有工作因子调整能力
- **scrypt** - 内存困难型算法，抗暴力破解能力强
- **Argon2** - 现代密码哈希算法，赢得密码哈希竞赛
  - Argon2i - 抗侧信道攻击
  - Argon2d - 高性能变体
  - Argon2id - 混合型变体
- **SHA-256** - 经典哈希算法
- **SHA-512** - 更长的哈希输出
- **MD5** - 向后兼容（不推荐用于新项目）
- 其他常见算法支持

## 技术特性

### 性能
- 使用 Rust 编写，具有高性能和低内存占用
- 快速的命令行启动时间
- 支持批量处理

### 安全性
- 遵循密码学最佳实践
- 自动生成加密安全的随机盐值
- 支持可配置的工作因子和迭代次数
- 防止常见的密码攻击

### 易用性
- 直观的命令行接口
- 清晰的帮助文档和示例
- 支持多种输出格式
- 易于脚本集成

## 使用场景

### 1. 开发环境
- 开发者在本地快速生成和测试密码哈希
- 集成到开发工作流和自动化脚本

### 2. 系统管理
- 系统管理员管理用户密码
- 批量生成和验证密码哈希

### 3. 测试和演示
- 为测试环境生成测试密码
- 演示密码安全最佳实践

### 4. 密码迁移
- 在系统之间迁移密码时进行哈希转换
- 验证迁移后的密码完整性

## CLI 命令设计

### 生成哈希
```bash
salt generate [OPTIONS] <PASSWORD>

选项:
  -a, --algorithm <ALGORITHM>    指定哈希算法 (默认: argon2id)
  -s, --salt <SALT>              自定义盐值 (可选，默认自动生成)
  -i, --iterations <N>           迭代次数 (算法相关)
  -w, --work-factor <N>          工作因子 (bcrypt/scrypt 相关)
  -m, --memory <MB>              内存成本 (Argon2 相关)
  -t, --time <N>                 时间成本 (Argon2 相关)
  -o, --output <FORMAT>          输出格式 (json/plain, 默认: plain)
  -h, --help                     显示帮助信息
```

### 验证密码
```bash
salt verify [OPTIONS] <PASSWORD> <HASH>

选项:
  -o, --output <FORMAT>          输出格式 (json/plain, 默认: plain)
  -v, --verbose                  详细输出
  -h, --help                     显示帮助信息
```

### 列出支持的算法
```bash
salt algorithms

输出: 列出所有支持的哈希算法及其参数说明
```

## 使用示例

### 示例 1: 使用默认算法生成哈希
```bash
$ salt generate "mypassword123"
$argon2id$v=19$m=19456,t=2,p=1$abcd1234efgh5678$xyz...
```

### 示例 2: 使用 bcrypt 生成哈希
```bash
$ salt generate -a bcrypt -w 12 "mypassword123"
$2b$12$R9h7cIPz0gi.URNNX3kh2O...
```

### 示例 3: 验证密码
```bash
$ salt verify "mypassword123" '$argon2id$v=19$m=19456,t=2,p=1$abcd1234efgh5678$xyz...'
✓ Password matches
```

### 示例 4: 验证失败的情况
```bash
$ salt verify "wrongpassword" '$argon2id$v=19$m=19456,t=2,p=1$abcd1234efgh5678$xyz...'
✗ Password does not match
```

### 示例 5: JSON 格式输出
```bash
$ salt generate -o json "mypassword123"
{
  "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$abcd1234efgh5678$xyz...",
  "algorithm": "argon2id",
  "salt": "abcd1234efgh5678",
  "timestamp": "2026-03-06T06:49:04Z"
}
```

## 安装

### 从源代码构建
```bash
git clone https://github.com/yourname/salt.git
cd salt
cargo build --release
```

### 从发行版安装
```bash
# 使用 cargo
cargo install salt

# 或从预编译二进制文件
# 下载对应平台的二进制文件
```

## 系统要求

- Rust 1.70+ (如果从源代码构建)
- 支持的操作系统: Linux, macOS, Windows
- 最小 RAM: 64MB
- 磁盘空间: ~10MB

## 输出格式

### Plain 格式 (默认)
```
$argon2id$v=19$m=19456,t=2,p=1$abcd1234efgh5678$xyz...
```

### JSON 格式
```json
{
  "password_hash": "...",
  "algorithm": "argon2id",
  "salt": "...",
  "parameters": {
    "memory_cost": 19456,
    "time_cost": 2,
    "parallelism": 1
  },
  "timestamp": "2026-03-06T06:49:04Z"
}
```

## 安全建议

1. **选择合适的算法**: 推荐使用 Argon2id 作为默认选择
2. **调整工作因子**: 根据安全需求和性能要求调整参数
3. **使用强密码**: 工具只能保护哈希过程，不能保证输入密码的强度
4. **定期更新**: 及时更新到最新版本以获得安全补丁
5. **避免在命令行传递密码**: 在生产环境中，考虑从标准输入读取密码

## 配置文件 ( v1.1 版本计划 )

用户可以创建 `~/config/salt/config.toml` 配置默认参数：

```toml
[defaults]
algorithm = "argon2id"
output_format = "plain"

[argon2id]
memory_cost = 19456
time_cost = 2
parallelism = 1

[bcrypt]
work_factor = 12

[scrypt]
n = 16384
r = 8
p = 1
```

## 许可证

MIT License

## 贡献指南

欢迎提交 Issue 和 Pull Request。请确保：
- 代码遵循 Rust 最佳实践
- 包含适当的测试用例
- 更新相关文档

## 路线图

### v1.0 (当前)
- ✓ 基础的哈希生成和验证
- ✓ 十余种算法支持
- ✓ CLI 接口

### v1.1 (计划中)
- 配置文件支持
- 批量处理模式
- 性能基准测试工具

### v2.0 (未来)
- 库接口 (crate)
- 图形用户界面
- 集成密码管理功能
