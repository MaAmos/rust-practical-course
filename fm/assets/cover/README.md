# 课程封面（fm 命令行工具）

已提供两套可直接使用的矢量封面（1920x1080）：
已提供以下尺寸的矢量封面：
- 1920×1080：cover-dark.svg（深色）、cover-light.svg（浅色）
- 192×128：cover-dark-192x128.svg（深色）、cover-light-192x128.svg（浅色）

特点：

导出建议（macOS）
 也可用 Inkscape：
  - 1920×1080 深色：`inkscape cover-dark.svg --export-type=png --export-filename=cover-dark.png -w 1920 -h 1080`
  - 1920×1080 浅色：`inkscape cover-light.svg --export-type=png --export-filename=cover-light.png -w 1920 -h 1080`
  - 192×128 深色：`inkscape cover-dark-192x128.svg --export-type=png --export-filename=cover-dark-192x128.png -w 192 -h 128`
  - 192×128 浅色：`inkscape cover-light-192x128.svg --export-type=png --export-filename=cover-light-192x128.png -w 192 -h 128`
  - 深色：`inkscape cover-dark.svg --export-type=png --export-filename=cover-dark.png -w 1920 -h 1080`
  - 浅色：`inkscape cover-light.svg --export-type=png --export-filename=cover-light.png -w 1920 -h 1080`
- 副标题：fm 命令行工具 · 文件查询 CLI（可替换为你的课程名）
- 右侧终端命令示例可按课程章节替换。
配色调整
- 关键配色集中在 `<linearGradient id="bg"/>`、`id="accent"` 等定义里，以及文本填充色；按需修改即可。
版权说明
- 本封面为项目课程用途，未使用 Rust 官方 Logo；可自由修改使用。
