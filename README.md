# WASI SSE Example

WASI-HTTP 环境下透传 SSE 请求的例子，其中包含接收 SSE 和发送 SSE 两个部分。

SSE 目录下是一个使用 node 编写的简单的 SSE 服务，Plugin 目录下是使用 Rust 和 Cargo Component 编写的 WASI-HTTP Component。

示例代码中使用的 WASI Runtime 是 Wasmtime。
