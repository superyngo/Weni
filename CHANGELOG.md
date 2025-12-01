# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-XX

### Added
- **系統資訊收集**
  - CPU 資訊（型號、核心數、使用率、頻率）
  - 記憶體資訊（總量、已用、可用、使用率）
  - 系統資訊（作業系統、內核版本、主機名）

- **硬體資訊收集**
  - 電池資訊（電量、充電狀態、健康度、溫度）
  - 磁盤資訊（掛載點、容量、檔案系統、使用率）
  - 網路資訊（介面名稱、傳輸/接收流量、封包統計）
  - 溫度監控（CPU 和組件溫度、最高溫度、臨界溫度）
  - 顯示器資訊（解析度、刷新率、縮放比例、多螢幕支援）

- **輸出功能**
  - 彩色表格輸出
  - JSON 格式輸出
  - 選擇性顯示特定資訊
  - 即時監控模式（`--watch`）
  - 自訂監控間隔（`--interval`）

- **跨平台支援**
  - Windows (x86_64, i686)
  - Linux (x86_64, i686, ARMv7, ARM64)
  - Linux musl (x86_64, i686, ARMv7, ARM64) - 靜態鏈接版本
  - macOS (Intel, Apple Silicon)

### Technical Details
- 使用 pico-args 實現輕量級 CLI 參數解析
- 使用 sysinfo 收集系統資訊
- 使用 battery crate 收集電池資訊
- 使用 display-info 收集顯示器資訊
- 使用 comfy-table 實現美觀的表格輸出
- 懶加載設計，僅收集請求的資訊
