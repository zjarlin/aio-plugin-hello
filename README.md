# Hello

此仓库是 AIO 最小全栈插件。`client` 和 `server` 提供 Rust 源码扩展，`runtime/pages.json` 提供可在线安装和卸载的 `PageDefinition` 产物。两种形态使用同一个 Git 提交作为发布与回滚单元。

```bash
aio plugin install <git>
```

健康接口为 `/api/plugins/aio-plugin-hello/health`。
