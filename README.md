# Weni

[English](#english) | [中文](#中文)

---

# English

A lightweight cross-platform system information CLI tool written in Rust.

## Installation

### Quick Install from GitHub Release (Recommended)

**Linux/macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/superyngo/weni/main/install.sh | bash
```

**Windows (PowerShell as Administrator):**

```powershell
irm https://raw.githubusercontent.com/superyngo/weni/main/install.ps1 | iex
```

The installer will:

- Download the latest pre-compiled binary from GitHub releases
- Install to `~/.local/bin` (Linux/macOS) or `%LOCALAPPDATA%\Programs\weni` (Windows)
- Add it to your PATH automatically

### Build from Source

```bash
# Clone the repository
git clone https://github.com/superyngo/weni.git
cd weni

# Build the release version
cargo build --release
```

The executable will be located at:

- Windows: `target/release/weni.exe`
- Linux/macOS: `target/release/weni`

You can manually copy it to your preferred directory in PATH.

## Uninstallation

**Linux/macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/superyngo/weni/main/install.sh | bash -s uninstall
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/superyngo/weni/main/install.ps1 | iex -Uninstall
```

Or manually remove:

- Linux/macOS: `rm ~/.local/bin/weni`
- Windows: Remove `%LOCALAPPDATA%\Programs\weni` directory

## Features

### Information Collection

- **CPU Information**: Model, cores, usage, frequency, architecture
- **Memory Information**: Total, used, available, usage percentage
- **System Information**: OS, kernel version, hostname, architecture
- **Battery Information**: Charge, state, health, temperature (if available)
- **Disk Information**: Mount points, capacity, filesystem, removable status
- **Network Information**: Interface names, transmitted/received traffic, packet stats, error counts
- **Temperature Monitoring**: CPU and component temperatures, max temp, critical temp (hardware dependent)
- **Process Information**: Running processes with CPU/memory usage, disk I/O
- **Hosts File**: Display hosts file entries with IP addresses and hostnames

### Display Features

- **Multiple Output Formats**: Colored tables, JSON
- **Live Monitoring Mode**: Auto-refresh information (htop-like)
- **Lazy Loading**: Only collect requested information for better performance
- **Selective Display**: Freely combine information types

## Usage

### Basic Usage

```bash
# Display all information (includes top 10 processes by CPU)
weni

# Display specific information
weni --cpu --memory
weni --system
weni --battery
weni --disk
weni --network
weni --temp
weni --process
weni --hosts

# Combine displays
weni --cpu --memory --disk --temp
```

### Process Monitoring

```bash
# Show all running processes
weni --process

# Show top 10 processes (sorted by memory)
weni --process --top 10

# Show top 10 processes sorted by CPU usage
weni --process --top 10 --sort-cpu
```

### Hosts File

```bash
# Display hosts file (comments filtered)
weni --hosts

# Display hosts file with comments
weni --hosts --show-comments
```

### JSON Output

```bash
# Output in JSON format (useful for scripting)
weni --json

# Output specific information as JSON
weni --cpu --memory --json
```

### Live Monitoring Mode

```bash
# Start monitoring mode (updates every 2 seconds)
weni --watch

# Custom update interval (5 seconds)
weni --watch --interval 5

# Monitor specific information
weni --cpu --memory --watch
```

### Help

```bash
weni --help
```

## Complete CLI Options

```
INFORMATION OPTIONS:
    --cpu                 Show CPU information
    --memory              Show memory information
    --system              Show system information
    --battery             Show battery information
    --disk                Show disk information
    --network             Show network information
    --temp                Show temperature information
    --process             Show running processes
    --hosts               Show hosts file contents

GENERAL OPTIONS:
    --json                Output in JSON format
    -w, --watch           Enable watch mode (live updates)
    -i, --interval <SEC>  Update interval in seconds (default: 2)
    -h, --help            Print help information

PROCESS OPTIONS:
    --top <N>             Show only top N processes (sorted by resource usage)
    --sort-cpu            Sort processes by CPU usage (default: by memory)

HOSTS OPTIONS:
    --show-comments       Show comments in hosts file (default: filter out)
```

## Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library entry point
├── cli.rs               # CLI argument parsing (pico-args)
├── collectors/          # Data collection modules
│   ├── mod.rs
│   ├── system.rs        # System information collector
│   ├── battery.rs       # Battery information collector
│   ├── disk.rs          # Disk information collector
│   ├── network.rs       # Network information collector
│   ├── temperature.rs   # Temperature information collector
│   ├── process.rs       # Process information collector
│   └── hosts.rs         # Hosts file collector
└── display/             # Output formatting modules
    ├── mod.rs
    └── formatter.rs     # Formatting logic
```

## Technology Stack

### Core Dependencies

- **sysinfo** (0.31): System information collection
- **battery** (0.7): Battery information collection
- **pico-args** (0.5): Lightweight CLI argument parsing

### Display & Serialization

- **serde** (1.0): Data serialization framework
- **serde_json** (1.0): JSON serialization
- **comfy-table** (7.1): Beautiful table display
- **colored** (2.1): Colored terminal output

### Utilities

- **anyhow** (1.0): Error handling

## Cross-Platform Support

✅ **Pre-compiled Binaries (12 Platforms)**

| Platform      | Architecture               | Description             |
| ------------- | -------------------------- | ----------------------- |
| Windows       | x86_64, i686               | 64-bit and 32-bit       |
| Linux (glibc) | x86_64, i686, ARMv7, ARM64 | Standard version        |
| Linux (musl)  | x86_64, i686, ARMv7, ARM64 | Static-linked version   |
| macOS         | x86_64, aarch64            | Intel and Apple Silicon |

## Examples

### Example 1: Quick System Status Check

```bash
weni --cpu --memory
```

### Example 2: Check Disk Space

```bash
weni --disk
```

### Example 3: Monitor System Performance

```bash
weni --cpu --memory --watch
```

### Example 4: Export Complete System Information to JSON

```bash
weni --json > system-info.json
```

### Example 5: Check Network Traffic

```bash
weni --network
```

### Example 6: Monitor System Temperature

```bash
weni --temp
# Note: Temperature monitoring requires hardware and OS support, Linux typically has better support
```

### Example 7: Monitor Running Processes

```bash
weni --process --top 10 --sort-cpu
```

### Example 8: View Hosts File

```bash
weni --hosts
```

## License

MIT License

---

# 中文

一個輕量級跨平台系統資訊 CLI 工具，使用 Rust 編寫。

## 安裝

### 從 GitHub Release 快速安裝（推薦）

**Linux/macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/superyngo/weni/main/install.sh | bash
```

**Windows (以管理員身份執行 PowerShell):**

```powershell
irm https://raw.githubusercontent.com/superyngo/weni/main/install.ps1 | iex
```

安裝程式將會：

- 從 GitHub releases 下載最新的預編譯二進制檔案
- 安裝到 `~/.local/bin` (Linux/macOS) 或 `%LOCALAPPDATA%\Programs\weni` (Windows)
- 自動加入到 PATH 環境變數

### 從原始碼編譯

```bash
# 複製儲存庫
git clone https://github.com/superyngo/weni.git
cd weni

# 編譯 release 版本
cargo build --release
```

執行檔位於：

- Windows: `target/release/weni.exe`
- Linux/macOS: `target/release/weni`

您可以手動複製到 PATH 中的任何目錄。

## 解除安裝

**Linux/macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/superyngo/weni/main/install.sh | bash -s uninstall
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/superyngo/weni/main/install.ps1 | iex -Uninstall
```

或手動移除：

- Linux/macOS: `rm ~/.local/bin/weni`
- Windows: 刪除 `%LOCALAPPDATA%\Programs\weni` 目錄

## 功能特性

### 資訊收集

- **CPU 資訊**: 型號、核心數、使用率、頻率、架構
- **記憶體資訊**: 總量、已用、可用、使用率
- **系統資訊**: 作業系統、內核版本、主機名、架構
- **電池資訊**: 電量、充電狀態、健康度、溫度（如有）
- **磁盤資訊**: 掛載點、容量、檔案系統、可移除性
- **網路資訊**: 介面名稱、傳輸/接收流量、封包統計、錯誤計數
- **溫度監控**: CPU 和組件溫度、最高溫度、臨界溫度（視硬體支援）
- **程序資訊**: 執行中程序的 CPU/記憶體使用、磁碟 I/O
- **Hosts 檔案**: 顯示 hosts 檔案的 IP 位址和主機名稱

### 顯示功能

- **多種輸出格式**: 彩色表格、JSON
- **即時監控模式**: 自動更新資訊（類似 htop）
- **懶加載**: 僅收集請求的資訊，提升效能
- **選擇性顯示**: 自由組合需要的資訊類型

## 使用方法

### 基本使用

```bash
# 顯示所有資訊（包含 top 10 程序依 CPU 排序）
weni

# 顯示特定資訊
weni --cpu --memory
weni --system
weni --battery
weni --disk
weni --network
weni --temp
weni --process
weni --hosts

# 組合顯示
weni --cpu --memory --disk --temp
```

### 程序監控

```bash
# 顯示所有執行中程序
weni --process

# 顯示前 10 個程序（依記憶體排序）
weni --process --top 10

# 顯示前 10 個程序（依 CPU 使用率排序）
weni --process --top 10 --sort-cpu
```

### Hosts 檔案

```bash
# 顯示 hosts 檔案（過濾註解）
weni --hosts

# 顯示 hosts 檔案（包含註解）
weni --hosts --show-comments
```

### JSON 輸出

```bash
# 輸出 JSON 格式（便於腳本整合）
weni --json

# 僅輸出特定資訊的 JSON
weni --cpu --memory --json
```

### 即時監控模式

```bash
# 啟動監控模式（每 2 秒更新一次）
weni --watch

# 自訂更新間隔（5 秒）
weni --watch --interval 5

# 監控特定資訊
weni --cpu --memory --watch
```

### 取得說明

```bash
weni --help
```

## 完整 CLI 選項

```
資訊選項:
    --cpu                 顯示 CPU 資訊
    --memory              顯示記憶體資訊
    --system              顯示系統資訊
    --battery             顯示電池資訊
    --disk                顯示磁盤資訊
    --network             顯示網路資訊
    --temp                顯示溫度資訊
    --process             顯示執行中程序
    --hosts               顯示 hosts 檔案內容

一般選項:
    --json                以 JSON 格式輸出
    -w, --watch           啟用監控模式（即時更新）
    -i, --interval <SEC>  更新間隔秒數（預設: 2）
    -h, --help            顯示說明資訊

程序選項:
    --top <N>             僅顯示前 N 個程序（依資源使用排序）
    --sort-cpu            依 CPU 使用率排序（預設：依記憶體）

Hosts 選項:
    --show-comments       顯示 hosts 檔案中的註解（預設：過濾）
```

## 專案架構

```
src/
├── main.rs              # CLI 入口點
├── lib.rs               # 函式庫入口
├── cli.rs               # CLI 參數解析 (pico-args)
├── collectors/          # 資料收集模組
│   ├── mod.rs
│   ├── system.rs        # 系統資訊收集器
│   ├── battery.rs       # 電池資訊收集器
│   ├── disk.rs          # 磁盤資訊收集器
│   ├── network.rs       # 網路資訊收集器
│   ├── temperature.rs   # 溫度資訊收集器
│   ├── process.rs       # 程序資訊收集器
│   └── hosts.rs         # Hosts 檔案收集器
└── display/             # 輸出格式化模組
    ├── mod.rs
    └── formatter.rs     # 格式化邏輯
```

## 技術棧

### 核心依賴

- **sysinfo** (0.31): 系統資訊收集
- **battery** (0.7): 電池資訊收集
- **pico-args** (0.5): 輕量級 CLI 參數解析

### 顯示與序列化

- **serde** (1.0): 資料序列化框架
- **serde_json** (1.0): JSON 序列化
- **comfy-table** (7.1): 美觀的表格顯示
- **colored** (2.1): 彩色終端輸出

### 工具

- **anyhow** (1.0): 錯誤處理

## 跨平台支援

✅ **預編譯版本 (12 個平台)**

| 平台          | 架構                       | 說明                   |
| ------------- | -------------------------- | ---------------------- |
| Windows       | x86_64, i686               | 64-bit 和 32-bit       |
| Linux (glibc) | x86_64, i686, ARMv7, ARM64 | 標準版本               |
| Linux (musl)  | x86_64, i686, ARMv7, ARM64 | 靜態鏈接版本           |
| macOS         | x86_64, aarch64            | Intel 和 Apple Silicon |

## 使用範例

### 範例 1: 快速檢視系統狀態

```bash
weni --cpu --memory
```

### 範例 2: 檢查磁盤空間

```bash
weni --disk
```

### 範例 3: 監控系統效能

```bash
weni --cpu --memory --watch
```

### 範例 4: 導出完整系統資訊到 JSON

```bash
weni --json > system-info.json
```

### 範例 5: 檢查網路流量

```bash
weni --network
```

### 範例 6: 監控系統溫度

```bash
weni --temp
# 注意: 溫度監控需要硬體和作業系統支援，Linux 系統通常有較好的支援
```

### 範例 7: 監控執行中程序

```bash
weni --process --top 10 --sort-cpu
```

### 範例 8: 檢視 Hosts 檔案

```bash
weni --hosts
```

## 授權

MIT License
