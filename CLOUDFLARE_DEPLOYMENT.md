# Cloudflare Pages 部署指南

本指南将帮助您将 subconverter-rs 部署到 Cloudflare Pages。

## 🚀 快速部署

### 方法一：通过 Cloudflare Dashboard

1. **Fork 此仓库** 到您的 GitHub 账户

2. **登录 Cloudflare Dashboard**
   - 访问 [Cloudflare Pages](https://pages.cloudflare.com/)
   - 点击 "Create a project"

3. **连接 GitHub 仓库**
   - 选择您 fork 的 `subconverter-rs` 仓库
   - 点击 "Begin setup"

4. **配置构建设置**
   ```
   Framework preset: Next.js
   Build command: ./scripts/build-cloudflare.sh
   Build output directory: www/.next
   Root directory: (留空)
   ```

5. **设置环境变量**
   ```
   NODE_ENV = production
   DEPLOY_ENV = cloudflare
   WASM_DEBUG = false
   ```

6. **部署**
   - 点击 "Save and Deploy"
   - 等待构建完成

### 方法二：通过 Wrangler CLI

1. **安装 Wrangler**
   ```bash
   npm install -g wrangler
   ```

2. **登录 Cloudflare**
   ```bash
   wrangler login
   ```

3. **构建项目**
   ```bash
   chmod +x scripts/build-cloudflare.sh
   ./scripts/build-cloudflare.sh
   ```

4. **部署到 Pages**
   ```bash
   cd www
   wrangler pages deploy .next --project-name subconverter-rs
   ```

## 🔧 配置说明

### 环境变量

在 Cloudflare Pages 设置中添加以下环境变量：

| 变量名 | 值 | 说明 |
|--------|----|----|
| `NODE_ENV` | `production` | Node.js 环境 |
| `DEPLOY_ENV` | `cloudflare` | 部署环境标识 |
| `WASM_DEBUG` | `false` | WASM 调试模式 |

### 自定义域名

1. 在 Cloudflare Pages 项目设置中
2. 点击 "Custom domains"
3. 添加您的域名
4. 按照提示配置 DNS

## 🛠️ 本地开发

1. **克隆仓库**
   ```bash
   git clone https://github.com/your-username/subconverter-rs.git
   cd subconverter-rs
   ```

2. **构建 WASM**
   ```bash
   ./scripts/build-cloudflare.sh
   ```

3. **启动开发服务器**
   ```bash
   cd www
   pnpm dev
   ```

## 📝 API 使用

部署完成后，您可以通过以下 API 端点使用服务：

### 订阅转换
```
GET https://your-domain.pages.dev/api/sub?target=clash&url=订阅链接
```

### 参数说明
- `target`: 目标格式 (clash, surge, v2ray, etc.)
- `url`: 原始订阅链接 (需要 URL 编码)
- `config`: 外部配置链接 (可选)

## 🔍 故障排除

### 常见问题

1. **WASM 加载失败**
   - 检查 `_headers` 文件是否正确配置
   - 确认 WASM 文件是否正确复制到输出目录

2. **构建失败**
   - 确认 Rust 和 wasm-pack 已正确安装
   - 检查构建脚本权限

3. **API 请求失败**
   - 检查 `_redirects` 文件配置
   - 确认环境变量设置正确

### 调试模式

启用调试模式：
1. 设置环境变量 `WASM_DEBUG=true`
2. 重新部署
3. 查看浏览器控制台日志

## 🚀 性能优化

1. **启用 Cloudflare 缓存**
   - 在 Page Rules 中设置缓存规则
   - 对静态资源启用长期缓存

2. **使用 Cloudflare CDN**
   - 自动启用全球 CDN 加速
   - 支持 HTTP/3 和 Brotli 压缩

3. **监控性能**
   - 使用 Cloudflare Analytics
   - 监控 API 响应时间

## 📞 支持

如果遇到问题，请：
1. 查看 [项目文档](README.md)
2. 提交 [GitHub Issue](https://github.com/lonelam/subconverter-rs/issues)
3. 加入 [Telegram 群组](https://t.me/subconverter_rs)
