# Cloudflare Pages 部署指南

## 🚀 快速部署步骤

### 方法一：通过 Cloudflare Dashboard（推荐）

1. **登录 Cloudflare Pages**
   - 访问 https://pages.cloudflare.com/
   - 点击 "Create a project"

2. **连接 GitHub 仓库**
   - 选择 `zhuyty/cfpagesub` 仓库
   - 点击 "Begin setup"

**注意：** 项目现在包含 `.pages.toml` 配置文件，Cloudflare Pages 会自动读取这个配置。

3. **配置构建设置**
   ```
   Framework preset: Next.js
   Build command: chmod +x build-cf.sh && ./build-cf.sh
   Build output directory: www/.next
   Root directory: (留空)
   Node.js version: 20
   ```

   **重要提示：** 不要使用 wrangler.toml 中的 build 配置，直接在 Cloudflare Pages Dashboard 中配置构建命令。

4. **设置环境变量**
   在 "Environment variables" 部分添加：
   ```
   NODE_ENV = production
   DEPLOY_ENV = cloudflare
   WASM_DEBUG = false
   NODE_VERSION = 20
   PNPM_VERSION = 9
   ```

5. **部署**
   - 点击 "Save and Deploy"
   - 等待构建完成（大约 5-10 分钟）

### 方法二：手动配置（推荐）

如果自动配置失败，请手动设置以下配置：

**构建命令：**
```bash
cd www && npm install && npm run build
```

**输出目录：**
```
www/.next
```

**环境变量：**
```
NODE_VERSION=20
NODE_ENV=production
DEPLOY_ENV=cloudflare
```

### 方法三：使用 pnpm（如果需要）

**构建命令：**
```bash
cd www && corepack enable && corepack prepare pnpm@latest --activate && pnpm install && pnpm build
```

**输出目录：**
```
www/.next
```

**环境变量：**
```
NODE_VERSION=20
NODE_ENV=production
DEPLOY_ENV=cloudflare
```

## 🔧 故障排除

### 常见问题及解决方案

1. **权限错误 (Permission denied)**
   - 确保构建命令包含 `chmod +x build-cf.sh`
   - 或使用简化的构建命令

2. **WASM 构建失败**
   - 检查 Rust 工具链是否正确安装
   - 确认 wasm-pack 版本兼容性

3. **Next.js 构建失败**
   - 确认 Node.js 版本为 20+
   - 检查 pnpm 是否正确安装

4. **依赖安装失败**
   - 清除缓存重新构建
   - 检查网络连接

### 调试步骤

1. **查看构建日志**
   - 在 Cloudflare Pages 控制台查看详细日志
   - 定位具体错误信息

2. **本地测试**
   ```bash
   # 克隆仓库
   git clone https://github.com/zhuyty/cfpagesub.git
   cd cfpagesub
   
   # 运行构建脚本
   chmod +x build-cf.sh
   ./build-cf.sh
   ```

3. **检查输出**
   - 确认 `www/.next` 目录存在
   - 检查文件结构是否正确

## 📝 API 使用

部署成功后，可通过以下方式使用：

```
https://your-project.pages.dev/api/sub?target=clash&url=订阅链接
```

## 🎯 性能优化

1. **启用缓存**
   - 在 Cloudflare 设置中启用缓存规则
   - 对静态资源设置长期缓存

2. **使用自定义域名**
   - 在项目设置中添加自定义域名
   - 配置 DNS 记录

3. **监控性能**
   - 使用 Cloudflare Analytics
   - 监控 API 响应时间和错误率

## 📞 获取帮助

如果遇到问题：
1. 检查 GitHub Issues
2. 查看 Cloudflare Pages 文档
3. 联系项目维护者
