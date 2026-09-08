# Hello

此仓库是 AIO 最小全栈插件。`client` 和 `server` 提供 Rust 源码扩展，`component` 将 `PageDefinition` 编译为 `aio:plugin/page@1` Wasm Component。宿主用 Wasmtime 校验并实例化组件，安装、停用、卸载和回滚都不需要重建宿主。

```bash
aio plugin install <git>
```

健康接口为 `/api/plugins/aio-plugin-hello/health`。
