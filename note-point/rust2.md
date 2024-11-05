# rust

- Reference

  [github-org: tyr-rust-bootcamp](https://github.com/tyr-rust-bootcamp), [template](https://github.com/tyr-rust-bootcamp/template), 

  



## 背景介绍

- rust训练营

  



### 基础概念

#### 版本协作 git





#### 开源协议 

- Open Source License

  定义了*软件的作者或版权持有人*与*用户*之间的权利和义务

- 常见协议

  1. **GPL (GNU General Public License)**: 基于GPL许可下的 都必须再次以GPL发布
  2. **LGPL (GNU Lesser General Public License)**: GPL变种 允许其他软件不需公开自身源代码
  3. **MIT License**: 几乎允许用户做任何事情 只需发布时包含原始的版权通知和许可声明即可
  4. **Apache License 2.0**: 类似于MIT条款 但还包含对专利权的明确授予(专利技术 无需支付额外费用)
  5. **BSD (Berkeley Software Distribution) Licenses**: 允许自由使用修改和再分发 但对分发软件有基本要求(保留版权声明)
  6. **CC0 (Creative Commons Zero)**: 不是典型的软件许可 而是放弃所有可能的版权和相关权利(希望作品完全无限制地开放给公众)

  



### 环境安装

#### rust编译器

- 安装

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
  ```

  



#### 工程化开发

- [cargo generate](https://github.com/cargo-generate/cargo-generate): 使用已有的 github repo 作为模版生成新的项目

  ```bash
  cargo install cargo-generate  # 源代码下载 本机编译 二进制文件运行
  cargo generate tyr-rust-bootcamp/template  # 使用模板
  
  
  cd ~/.cargo/bin/ && ls
  #cargo-clippy.exe  cargo-miri.exe  clippy-driver.exe  rust-analyzer.exe  rust-gdbgui.exe  rustc.exe    rustfmt.exe
  #cargo-fmt.exe     cargo.exe       rls.exe            rust-gdb.exe       rust-lldb.exe    rustdoc.exe  rustup.exe
  ls ~/.cargo/bin/ | grep generate
  #cargo-generate.exe
  
  ```

- [pre-commit](https://github.com/pre-commit/pre-commit): 在提交代码前进行代码检查

  ```bash
  pip install pre-commit  # 全局安装工具
  pre-commit install  # 在每个项目 显式安装
  
  
  # 配置文件 .pre-commit-config.yaml
  # https://pre-commit.com/
  
  ```

  .pre-commit-config.yaml

  ```yaml
  
  ```

- Cargo deny: 检查依赖的安全性 (生产环境 企业级项目)

  ```bash
  cargo install --locked cargo-deny
  
  ```

- typos: 拼写检查工具

  ```bash
  cargo install typos-cli
  
  ```

- git cliff: 生成 changelog 

  ```bash
  cargo install git-cliff
  
  ```
  
- cargo nextest: Rust 增强测试工具

  ```bash
  cargo install cargo-nextest --locked
  
  ```

  



#### vscode

- vscode-plugin

  crates: Rust 包管理

  Even Better TOML: TOML 文件支持

  Better Comments: 优化注释显示

  Error Lens: 错误提示优化

  GitLens: Git 增强

  Github Copilot: 代码提示

  indent-rainbow: 缩进显示优化

  Prettier - Code formatter: 代码格式化

  REST client: REST API 调试

  rust-analyzer: Rust 语言支持

  Rust Test lens: Rust 测试支持

  Rust Test Explorer: Rust 测试概览

  TODO Highlight: TODO 高亮

  vscode-icons: 图标优化

  YAML: YAML 文件支持







































































