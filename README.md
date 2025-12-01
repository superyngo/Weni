# Weni

一個輕量級跨平台系統資訊 CLI 工具，使用 Rust 編寫。

## 功能特性

### 資訊收集
- **CPU 資訊**: 型號、核心數、使用率、頻率
- **記憶體資訊**: 總量、已用、可用、使用率
- **系統資訊**: 作業系統、內核版本、主機名
- **電池資訊**: 電量、充電狀態、健康度、溫度（如有）
- **磁盤資訊**: 掛載點、容量、檔案系統、可移除性
- **網路資訊**: 介面名稱、傳輸/接收流量、封包統計、錯誤計數
- **溫度監控**: CPU 和組件溫度、最高溫度、臨界溫度（視硬體支援）
- **顯示器資訊**: 解析度、刷新率、縮放比例、多螢幕支援、主顯示器標識

### 顯示功能
- **多種輸出格式**: 彩色表格、JSON
- **即時監控模式**: 自動更新資訊（類似 htop）
- **懶加載**: 僅收集請求的資訊，提升效能
- **選擇性顯示**: 自由組合需要的資訊類型

## 安裝

```bash
cargo build --release
```

執行檔位於 `target/release/weni.exe` (Windows) 或 `target/release/weni` (Linux/macOS)

## 使用方法

### 基本使用

```bash
# 顯示所有資訊
weni

# 顯示特定資訊
weni --cpu --memory
weni --system
weni --battery
weni --disk
weni --network
weni --temp
weni --display

# 組合顯示
weni --cpu --memory --disk --display
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
OPTIONS:
    --cpu                 顯示 CPU 資訊
    --memory              顯示記憶體資訊
    --system              顯示系統資訊
    --battery             顯示電池資訊
    --disk                顯示磁盤資訊
    --network             顯示網路資訊
    --temp                顯示溫度資訊
    --display             顯示顯示器/螢幕資訊
    --json                以 JSON 格式輸出
    -w, --watch           啟用監控模式（即時更新）
    -i, --interval <SEC>  更新間隔秒數（預設: 2）
    -h, --help            顯示說明資訊
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
│   └── display.rs       # 顯示器資訊收集器
└── display/             # 輸出格式化模組
    ├── mod.rs
    └── formatter.rs     # 格式化邏輯
```

## 技術棧

### 核心依賴
- **sysinfo** (0.31): 系統資訊收集
- **battery** (0.7): 電池資訊收集
- **display-info** (0.5): 顯示器資訊收集
- **pico-args** (0.5): 輕量級 CLI 參數解析

### 顯示與序列化
- **serde** (1.0): 資料序列化框架
- **serde_json** (1.0): JSON 序列化
- **comfy-table** (7.1): 美觀的表格顯示
- **colored** (2.1): 彩色終端輸出

### 工具
- **anyhow** (1.0): 錯誤處理

## 跨平台支援

✅ **Windows**
  - x86_64 (64-bit)
  - i686 (32-bit)

✅ **Linux**
  - x86_64 (64-bit, glibc)
  - i686 (32-bit, glibc)
  - ARMv7 (32-bit ARM)
  - ARM64 (aarch64)

✅ **Linux musl**（靜態鏈接版本）
  - x86_64-musl
  - i686-musl
  - ARMv7-musl
  - ARM64-musl

✅ **macOS**
  - x86_64 (Intel)
  - aarch64 (Apple Silicon)

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

### 範例 7: 查看顯示器配置
```bash
weni --display
# 適用於多螢幕設定，顯示所有顯示器的解析度、刷新率、位置等
```

## 授權

MIT License
