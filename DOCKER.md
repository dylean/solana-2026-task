# Docker 环境使用指南

## 快速开始

### 1. 构建镜像

```bash
docker-compose build
```

### 2. 启动开发环境

```bash
# 启动开发容器
docker-compose run --rm solana-dev

# 或启动所有服务（包括测试验证器）
docker-compose up -d
```

### 3. 进入开发容器

```bash
docker exec -it solana-dev-env bash
```

## 使用场景

### 场景 1: 开发和构建

```bash
# 启动开发容器
docker-compose run --rm solana-dev

# 在容器内构建项目
cd Task2/blueshift_anchor_vault
anchor build
anchor test
```

### 场景 2: 使用本地验证器

```bash
# 启动验证器
docker-compose up -d solana-test-validator

# 在另一个终端启动开发容器
docker-compose run --rm solana-dev

# 配置连接到本地验证器
solana config set --url http://solana-test-validator:8899
```

### 场景 3: 持续开发

```bash
# 启动所有服务并保持运行
docker-compose up -d

# 进入开发容器
docker exec -it solana-dev-env bash

# 开发完成后停止
docker-compose down
```

## 镜像说明

### 包含的工具

- **Rust**: 1.92.0 (nightly)
- **Solana CLI**: 1.18.26
- **Anchor**: 0.32.1
- **Node.js**: 20.x
- **Yarn**: latest

### 环境配置

- Solana 默认连接到 devnet
- Cargo 缓存持久化
- 工作目录映射到 `/workspace`

## 常用命令

### Docker 管理

```bash
# 查看运行的容器
docker-compose ps

# 查看日志
docker-compose logs -f

# 停止所有服务
docker-compose down

# 清理所有数据（包括缓存）
docker-compose down -v
```

### 开发命令

```bash
# 在容器内执行单个命令
docker-compose run --rm solana-dev anchor build

# 在运行的容器内执行命令
docker exec solana-dev-env anchor test
```

## 故障排除

### 问题 1: 构建失败

```bash
# 清理并重新构建
docker-compose down -v
docker-compose build --no-cache
```

### 问题 2: 权限问题

```bash
# 修改文件所有权
docker-compose run --rm solana-dev chown -R $(id -u):$(id -g) /workspace
```

### 问题 3: 端口冲突

如果端口 8899 或 8900 已被占用，修改 `docker-compose.yml`：

```yaml
ports:
  - "18899:8899"  # 使用不同的主机端口
  - "18900:8900"
```

## 性能优化

### 使用缓存卷

Docker Compose 配置已包含缓存卷：
- `cargo-cache`: Rust 依赖缓存
- `solana-cache`: Solana 程序缓存
- `validator-ledger`: 验证器账本数据

### 加速构建

```bash
# 使用 BuildKit
DOCKER_BUILDKIT=1 docker-compose build
```

## 生产环境注意事项

⚠️ **警告**: 此 Docker 配置仅用于开发环境，不适合生产部署。

生产环境建议：
1. 使用专用的 Solana 验证器节点
2. 配置适当的安全策略
3. 使用 secrets 管理敏感信息
4. 实施监控和日志收集

## 与本地环境对比

| 特性 | Docker 环境 | 本地环境 |
|------|------------|---------|
| 环境一致性 | ✅ 完全一致 | ❌ 可能不同 |
| 安装复杂度 | ✅ 简单 | ❌ 复杂 |
| 启动速度 | ⚠️ 较慢 | ✅ 快速 |
| 资源占用 | ⚠️ 较高 | ✅ 较低 |
| 隔离性 | ✅ 完全隔离 | ❌ 共享系统 |
| 适用场景 | 团队协作、CI/CD | 个人开发 |

## 推荐工作流

1. **初次设置**: 使用 Docker 确保环境一致
2. **日常开发**: 根据个人喜好选择 Docker 或本地环境
3. **CI/CD**: 使用 Docker 确保构建可重现
4. **问题调试**: 在 Docker 环境中复现和修复

## 参考资料

- [Docker 官方文档](https://docs.docker.com/)
- [Docker Compose 文档](https://docs.docker.com/compose/)
- [Solana Docker 最佳实践](https://docs.solana.com/developing/on-chain-programs/developing-rust#docker)
