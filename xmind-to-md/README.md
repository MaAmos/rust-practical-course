# XMind to Markdown Converter

一款轻量级、纯前端实现的 XMind 思维导图转 Markdown 工具。

## ✨ 特性

- **即时转换**: 拖拽即看，无需上传服务器，保护隐私。
- **层级映射**: 自动将 XMind 的树状结构转换为符合逻辑的 Markdown 标题层级。
- **笔记支持**: 完美支持 XMind 中的节点笔记转换。
- **离线使用**: 纯客户端逻辑，支持静态部署，不消耗服务器资源。
- **预览导出**: 提供转换后的 Markdown 实时预览，并支持一键下载。

## 🚀 快速启动

1. **安装依赖**:

   ```bash
   npm install
   ```

2. **启动开发服务器**:

   ```bash
   npm run dev
   ```

3. **构建发布**:
   ```bash
   npm run build
   ```

## 📂 项目结构

- `src/utils/xmindParser.js`: XMind 核心解析引擎。
- `src/utils/markdownGenerator.js`: Markdown 转换逻辑。
- `src/components/`: UI 组件库（DropZone 拖拽区、ConverterView 转换视图等）。

## 🛠 开发计划

- [ ] 支持 XMind 中的标签同步。
- [ ] 支持更多 Markdown 导出样式模板。
- [ ] 增加多思维导图合并导出功能。
