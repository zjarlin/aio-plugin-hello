# Hello

此仓库是 AIO 最小全栈插件。`component` 将页面定义、按钮事件和轻量请求处理器编译为 `aio:plugin/page@1` Wasm Component。宿主用 Wasmtime 在无 WASI 权限的环境中校验并实例化组件，安装、停用、卸载和回滚都不需要重建宿主。

```bash
cargo test --workspace
cargo build --release --target wasm32-unknown-unknown -p aio-plugin-hello-component
wasm-tools component new \
  target/wasm32-unknown-unknown/release/aio_plugin_hello_component.wasm \
  -o dist/hello.wasm
aio plugin validate
aio plugin publish
```
