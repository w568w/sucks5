# Sucks5

> Life sucks, but it's better with Sucks5. - Me

（中文版请翻到下方）

Sucks5 is a simple, lightweight proxy client for SOCKS protocol in Rust. It is mainly designed for connecting to a SOCKS server with standard input/output, e.g., using `ssh` command over a SOCKS proxy.

## Features
- Connects to a SOCKS server with standard input/output.
- Supports SOCKS4, SOCKS4a and SOCKS5 protocols (optional authentication).
- Seamless integration with `ssh` command.
- Supports all major platforms that Rust supports: Windows, macOS, Linux, Android, iOS, and more (you name it).

## Installation
### From Source

1. Install Rust toolchain from [rustup.rs](https://rustup.rs/).
2. Run `cargo install --git https://github.com/w568w/sucks5`.

### Pre-built Binaries
Pre-built binaries are available on the [Releases](https://github.com/w568w/sucks5/releases) page. Only Windows binaries are provided for now. If you need binaries for other platforms, please build from source.

## Usage
Too simple to be explained. Just run `sucks5` and look at the help message.


## Why Sucks5?
Here is a dull story. I was trying to use `ssh` command to connect to a remote server over a SOCKS5 proxy, so I googled "**ssh socks5 proxy on Windows**". Surprisingly, got few results and none of them worked:

- **GNU/BSD `nc` and `socat`**: Not available on Windows, and both don't support SOCKS5 with authentication.
- **Nmap's `ncat`**: [An 7-year-old issue that makes it unusable](https://github.com/nmap/nmap/issues/1026) on Windows is still there. Nobody cares.
- **`connect.exe` carried by MinGW**: also very limited and doesn't support SOCKS5 with authentication, and as slow as ncat.
- **Cygwin**: it has a least-funcional `nc` and `socat`, and it's a awful pain to install and use. Also, it doesn't support SOCKS5 with authentication.

I was like, "*What the heck? Nobody uses SOCKS5 on Windows all over the world except me?*" So I decided to write a simple SOCKS5 client in Rust. And here it is, Sucks5.

# Sucks5

> Life sucks, but it's better with Sucks5. ——我自己说的

Sucks5 是一个简单、轻量的 Rust 实现的 SOCKS 协议代理客户端。它主要用于通过标准输入/输出连接到 SOCKS 服务器，例如使用 `ssh` 命令通过 SOCKS 代理连接到远程服务器。

## 特性
- 通过标准输入/输出连接到 SOCKS 服务器。
- 支持 SOCKS4、SOCKS4a 和 SOCKS5 协议。
- 与 `ssh` 命令无缝集成。
- 支持 Rust 支持的所有主流平台：Windows、macOS、Linux、Android、iOS 等等（你说了算）。

## 安装
### 从源码安装
1. 从 [rustup.rs](https://rustup.rs/) 安装 Rust 工具链。
2. 运行 `cargo install --git https://github.com/w568w/sucks5`。

### 预编译二进制文件
预编译二进制文件可以在 [Releases](https://github.com/w568w/sucks5/releases) 页面找到。目前只提供了 Windows 平台的二进制文件。如果你需要其他平台的二进制文件，请自行编译。

## 使用
太简单了，不用解释。只需运行 `sucks5`，然后查看帮助信息你就知道了。

## 为什么要 Sucks5？
这是一个无聊的故事。我想使用 `ssh` 命令通过 SOCKS5 代理连接到远程服务器，于是我谷歌搜索了一下：“**Windows ssh socks5 代理**”。令人惊讶的是，搜索结果很少，而且没有一个能用的：

- **GNU/BSD 的 `nc` 和 `socat`**：Windows 上没有，而且都不支持带认证的 SOCKS5。
- **Nmap 的 `ncat`**：Windows 上存在一个 [7 年前的 Bug](https://github.com/nmap/nmap/issues/1026)，导致它无法使用。根本没有人关心修复。
- **MinGW 自带的 `connect.exe`**：也非常有限，不支持带认证的 SOCKS5，而且和 ncat 一样极慢。
- **Cygwin**：它有一个功能阉割的 `nc` 和 `socat`，而且安装和使用都非常痛苦。而且它也不支持带认证的 SOCKS5。

我当时就在想，“*什么鬼？全世界除了我之外，难道没有人在 Windows 上使用 SOCKS5 代理吗？*”于是我决定用 Rust 写一个简单的 SOCKS5 客户端。于是就有了 Sucks5。