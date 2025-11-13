# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

这是一个 Hyperliquid 交易平台的 Rust SDK 扩展库,提供了完整的交易 API、信息查询和 WebSocket 订阅功能。

## 常用命令

### 构建和运行
```bash
# 构建项目
cargo build

# 运行测试(在 exchange_client.rs 中)
cargo test

# 运行示例程序
cargo run --bin [EXAMPLE_NAME]
# 例如:
cargo run --bin info              # 信息查询示例
cargo run --bin order_and_cancel  # 订单操作示例
cargo run --bin market_maker      # 做市商示例
cargo run --bin ws_orders         # WebSocket 订单更新示例
```

### 示例程序位置
所有示例程序位于 `src/bin/` 目录,包括:
- 交易操作: `order_and_cancel.rs`, `market_order_and_cancel.rs`, `spot_order.rs`
- WebSocket 订阅: `ws_orders.rs`, `ws_trades.rs`, `ws_l2_book.rs`, `ws_candles.rs`
- 账户管理: `usdc_transfer.rs`, `class_transfer.rs`, `leverage.rs`
- 高级功能: `market_maker.rs`, `agent.rs`, `approve_builder_fee.rs`

## 核心架构

### 模块组织

1. **ExchangeClient** (`src/exchange/exchange_client.rs`)
   - 核心交易客户端,处理所有交易操作
   - 使用 `PrivateKeySigner` 进行签名认证
   - 支持 Mainnet/Testnet 环境切换
   - 主要功能:
     - 订单操作: `order()`, `bulk_order()`, `cancel()`, `modify()`
     - 市价单: `market_open()`, `market_close()`
     - 账户管理: `usdc_transfer()`, `class_transfer()`, `vault_transfer()`
     - 杠杆控制: `update_leverage()`, `update_isolated_margin()`
     - 高级功能: `approve_agent()`, `approve_builder_fee()`, `create_vault()`

2. **InfoClient** (`src/info/info_client.rs`)
   - 查询市场信息和账户状态
   - 支持 WebSocket 订阅
   - 主要功能:
     - 账户查询: `user_state()`, `open_orders()`, `user_fills()`
     - 市场数据: `meta()`, `all_mids()`, `l2_snapshot()`, `recent_trades()`
     - 历史数据: `funding_history()`, `candles_snapshot()`
     - WebSocket: `subscribe()`, `unsubscribe()`

3. **WebSocket Manager** (`src/ws/ws_manager.rs`)
   - 管理所有 WebSocket 连接和订阅
   - 支持断线重连(`reconnect` 参数)
   - 订阅类型: `AllMids`, `Trades`, `L2Book`, `UserEvents`, `OrderUpdates`, `Candle` 等

4. **Signature Module** (`src/signature/`)
   - EIP-712 签名实现
   - L1 操作签名: `sign_l1_action()`
   - 类型化数据签名: `sign_typed_data()`
   - 支持 Agent 授权机制

### 关键数据流

1. **订单提交流程**:
   - 用户创建 `ClientOrderRequest`
   - `ExchangeClient.order()` → 转换为 `OrderRequest` (包含 asset index)
   - 序列化为 `Actions::Order` → 计算哈希 → 签名
   - 发送到 `/exchange` 端点

2. **WebSocket 订阅流程**:
   - 创建 `InfoClient` 并调用 `subscribe()`
   - 自动创建 `WsManager` 实例
   - 消息通过 `UnboundedSender` 发送到用户 channel
   - 处理断线重连和订阅管理

3. **签名验证**:
   - 所有 L1 操作需要 `sign_l1_action()` 签名(Order, Cancel, Modify 等)
   - 跨链操作使用 `sign_typed_data()` (UsdSend, Withdraw3, SpotSend 等)
   - Mainnet 和 Testnet 使用不同的域分隔符

### 重要设计模式

1. **环境切换**: 使用 `BaseUrl` 枚举(Mainnet/Testnet/Local)控制所有端点
2. **资产映射**: `coin_to_asset` HashMap 将币种名称映射到 asset index
3. **Vault 支持**: 可选的 `vault_address` 参数支持 vault 操作
4. **Builder 集成**: 订单支持 builder fee 和 grouping
5. **错误处理**: 统一的 `Result<T>` 类型,使用 `Error` 枚举

### 测试

- 单元测试位于 `src/exchange/exchange_client.rs` 底部
- 测试覆盖签名验证、action 哈希计算
- 示例程序也可作为集成测试参考

## 开发注意事项

1. **私钥安全**: 示例代码中的私钥仅用于演示,生产环境需使用环境变量
2. **精度处理**:
   - 价格和数量使用 `sz_decimals` 和 `price_decimals`
   - 使用 `round_to_decimals()` 和 `truncate_float()` 处理浮点数
3. **时间戳**: 使用 `next_nonce()` 生成 millisecond 级时间戳
4. **订单类型**:
   - Limit: `tif` 可为 "Gtc", "Ioc", "Alo"
   - Trigger: 需要 `trigger_px`, `is_market`, `tpsl`
5. **WebSocket 重连**: 使用 `InfoClient::with_reconnect()` 启用自动重连