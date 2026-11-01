# 经验教训 (LESSONS.md)

- **Windows 下 ghostty-sys 依赖 libclang**：`crates/ghostty-sys/build.rs` 使用 bindgen 生成 bindings 时，Windows 环境常因未全局安装 LLVM 而报 `Unable to find libclang`。修法：在 `build.rs` 中自动检索 `uv` / `pip install libclang` 的原生动态库路径，并检入预生成的 `crates/ghostty-sys/src/bindings.rs` 作为安全回退。
- **Windows CRLF 导致 Bun 测试语法错误**：`scripts/*.test.ts` 中存在使用 `replace(/^import .*;\n/gm, "")` 剥离模块 import 的逻辑，Windows 下若检出为 CRLF (`\r\n`) 会导致正则未匹配而将 import 代码传入 `new Function()` 引发语法错误。必须保持 `core.autocrlf = false`（LF 换行）。
- **THIRD-PARTY-NOTICES 校验源文件哈希**：`tests/notices.rs` 会断言 `crates/ghostty-sys/build.rs` 等构建文件的 SHA256，若修改了此类文件，必须同步更新 `THIRD-PARTY-NOTICES` 中的相应 SHA256 记录，否则测试会失败。
- **部分 UI 文本被单元测试断言固化**：如 `defaults/mods/spaces/main.js` 中的 `"rebuilt ·"` 以及设置更新界面的 `"Up to date"`、`"Checking"`，在汉化时需注意保留原判定词或在双语形式下确保包含原字符串，避免破坏上游单元测试。
