# Salt v1.0 开发 TODO

> 基于 [project-definition.md](./project-definition.md) 的 v1.0 功能规划

## 项目初始化

- [x] 创建 Rust 项目结构 ✅
- [x] 添加必要的依赖 ✅
  - [x] `clap` - CLI 参数解析 ✅
  - [x] `argon2` - Argon2 算法支持 ✅
  - [x] `bcrypt` - bcrypt 算法支持 ✅
  - [x] `pbkdf2` - PBKDF2 算法支持 ✅
  - [x] `scrypt` - scrypt 算法支持 ✅
  - [x] `sha2` - SHA-256/SHA-512 支持 ✅
  - [x] `md5` - MD5 支持 (向后兼容) ✅
  - [x] `rand` - 安全随机数生成 ✅
  - [x] `serde` + `serde_json` - JSON 输出格式 ✅
  - [x] `thiserror` / `anyhow` - 错误处理 ✅

## 核心模块设计

- [x] 设计错误类型模块 (`src/error.rs`) ✅
  - [x] 定义统一的错误枚举 ✅
  - [x] 实现各算法的错误转换 ✅
  - [x] 提供友好的错误信息 ✅

- [x] 设计算法抽象层 (`src/hasher/mod.rs`) ✅
  - [x] 定义 `Hasher` trait ✅
  - [x] 定义算法配置结构体 ✅
  - [x] 实现算法注册表 ✅

- [x] 设计输出格式模块 (`src/output.rs`) ✅
  - [x] 定义 `OutputFormat` 枚举 (Plain/JSON) ✅
  - [x] 实现 JSON 序列化结构 ✅
  - [x] 实现格式化输出函数 ✅

## 算法实现

### Argon2 系列
- [x] 实现 Argon2i (`src/hasher/argon2.rs`) ✅
- [x] 实现 Argon2d (`src/hasher/argon2.rs`) ✅
- [x] 实现 Argon2id (`src/hasher/argon2.rs`) ✅
- [x] 支持参数配置: memory_cost, time_cost, parallelism ✅

### bcrypt
- [x] 实现 bcrypt (`src/hasher/bcrypt.rs`) ✅
- [x] 支持 work_factor 配置 ✅

### scrypt
- [x] 实现 scrypt (`src/hasher/scrypt.rs`) ✅
- [x] 支持参数配置: n, r, p ✅

### PBKDF2
- [x] 实现 PBKDF2 (`src/hasher/pbkdf2.rs`) ✅
- [x] 支持迭代次数配置 ✅

### 经典哈希 (用于向后兼容)
- [x] 实现 SHA-256 (`src/hasher/classic.rs`) ✅
- [x] 实现 SHA-512 (`src/hasher/classic.rs`) ✅
- [x] 实现 MD5 (`src/hasher/classic.rs`) ✅

## CLI 实现

- [x] 实现主命令结构 (`src/main.rs`) ✅
  - [x] 使用 `clap` 定义命令行接口 ✅
  - [x] 定义全局选项 ✅

- [x] 实现 `generate` 子命令 (`src/main.rs`) ✅
  - [x] 解析密码参数 (支持位置参数) ✅
  - [x] 解析算法选项 (-a, --algorithm) ✅
  - [x] 解析盐值选项 (-s, --salt) ✅
  - [x] 解析迭代次数 (-i, --iterations) ✅
  - [x] 解析工作因子 (-w, --work-factor) ✅
  - [x] 解析内存成本 (-m, --memory) ✅
  - [x] 解析时间成本 (-t, --time) ✅
  - [x] 解析输出格式 (-o, --output) ✅
  - [x] 调用对应算法生成哈希 ✅
  - [x] 格式化并输出结果 ✅

- [x] 实现 `verify` 子命令 (`src/main.rs`) ✅
  - [x] 解析密码和哈希参数 ✅
  - [x] 自动识别哈希算法 ✅
  - [x] 执行验证逻辑 ✅
  - [x] 格式化输出验证结果 ✅
  - [x] 支持详细输出模式 (-v, --verbose) ✅

- [x] 实现 `algorithms` 子命令 (`src/main.rs`) ✅
  - [x] 列出所有支持的算法 ✅
  - [x] 显示各算法的参数说明 ✅
  - [x] 格式化输出算法信息表 ✅

## 安全功能

- [x] 实现安全盐值生成 ✅
  - [x] 使用 `rand` 生成加密安全随机盐 ✅
  - [x] 支持自定义盐值长度 ✅
  - [x] 盐值编码 (Base64/Hex) ✅

- [x] 实现密码输入安全 ✅
  - [x] 支持从标准输入读取密码 ✅
  - [x] 支持交互式密码输入 (隐藏输入) ✅

## 测试

- [x] 单元测试 ✅
  - [x] 各算法生成测试 ✅
  - [x] 各算法验证测试 ✅
  - [x] 错误处理测试 ✅
  - [x] 边界条件测试 ✅

- [x] 集成测试 ✅
  - [x] CLI 命令端到端测试 ✅
  - [x] 跨算法兼容性测试 ✅
  - [x] JSON 输出格式验证 ✅

## 文档

- [x] 完善 README.md ✅
- [x] 代码文档 ✅
- [x] 用户文档 ✅
  - [x] 创建 `docs/usage.md` 使用手册 ✅
  - [x] 创建 `docs/algorithms.md` 算法说明文档 ✅
  - [x] 创建 `docs/security.md` 安全建议文档 ✅

## 发布准备

- [x] 版本号确认 (`Cargo.toml`) ✅
- [x] 添加 LICENSE 文件 (MIT) ✅
- [x] 添加 CHANGELOG.md ✅

## 进度追踪

| 模块 | 状态 | 完成度 |
|------|------|--------|
| 项目初始化 | ✅ 已完成 | 100% |
| 核心模块设计 | ✅ 已完成 | 100% |
| 算法实现 | ✅ 已完成 | 100% |
| CLI 实现 | ✅ 已完成 | 100% |
| 安全功能 | ✅ 已完成 | 100% |
| 测试 | ✅ 已完成 | 100% |
| 文档 | ✅ 已完成 | 100% |
| 发布准备 | ✅ 已完成 | 100% |

---

**图例说明:**
- ✅ 已完成
- 🚧 进行中
- ⏳ 待开始
- ⏸️ 暂停/阻塞

**优先级说明:**
- P0: 核心功能，必须完成
- P1: 重要功能，建议完成
- P2: 增强功能，可选完成
