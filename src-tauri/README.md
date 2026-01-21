# DailyCraft 后端架构设计

## 目录结构

```
src-tauri/src/
├── main.rs              # 入口，只做启动
├── lib.rs               # 注册所有模块
├── commands/            # 暴露给前端的命令
│   ├── mod.rs
│   ├── fs.rs            # 文件/日志/本地IO
│   ├── system.rs        # 系统信息/权限/窗口监控
│   ├── ai.rs            # AI调用接口
│   └── plugin.rs        # 插件管理命令
│
├── services/            # 纯Rust业务（不依赖tauri）
│   ├── mod.rs
│   ├── log_parser.rs    # 日志解析
│   ├── analyzer.rs      # 活动分析
│   ├── storage.rs       # 数据存储(SQLite/JSON)
│   └── window_tracker.rs # 窗口追踪服务
│
├── plugins/             # 插件系统
│   ├── mod.rs           # 插件trait定义
│   ├── loader.rs        # 动态加载插件
│   ├── registry.rs      # 插件注册表
│   └── sandbox.rs       # 插件沙箱/权限控制
│
├── ai/                  # AI模块
│   ├── mod.rs
│   ├── provider.rs      # AI提供商trait (OpenAI/Claude/本地)
│   ├── diary.rs         # 日记生成
│   └── embeddings.rs    # 向量嵌入(可选)
│
├── models/              # 数据模型
│   ├── mod.rs
│   ├── activity.rs      # 活动记录
│   ├── card.rs          # 卡片数据
│   └── config.rs        # 配置结构
│
├── state.rs             # 全局状态管理
└── error.rs             # 统一错误定义
```

## 模块职责

### commands/ - 前端命令层
暴露给前端调用的Tauri命令，只做参数校验和调用services层。

| 文件 | 职责 |
|------|------|
| `fs.rs` | 文件读写、日志导出、本地IO操作 |
| `system.rs` | 系统信息获取、权限申请、窗口监控 |
| `ai.rs` | AI接口调用、日记生成 |
| `plugin.rs` | 插件安装/卸载/启用/禁用 |

### services/ - 业务逻辑层
纯Rust业务逻辑，不依赖Tauri，方便单元测试。

| 文件 | 职责 |
|------|------|
| `log_parser.rs` | 解析活动日志 |
| `analyzer.rs` | 分析用户活动模式 |
| `storage.rs` | 数据持久化(SQLite/JSON) |
| `window_tracker.rs` | 监控当前活动窗口 |

### plugins/ - 插件系统
支持社区开发的插件扩展。

```rust
// 插件trait示例
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_activity(&self, activity: &Activity) -> Option<CardData>;
    fn on_generate_diary(&self, activities: &[Activity]) -> Option<String>;
}
```

### ai/ - AI模块
抽象AI调用，支持多种AI提供商。

```rust
// AI提供商trait示例
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate_diary(&self, activities: &[Activity]) -> Result<String>;
    async fn analyze_activity(&self, activity: &Activity) -> Result<Analysis>;
}
```

### models/ - 数据模型
统一的数据结构定义。

```rust
// 活动记录示例
#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub app_name: String,
    pub window_title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration: Duration,
}

// 卡片数据示例
#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub activity_id: String,
    pub position: Position,
    pub color: String,
    pub tags: Vec<String>,
}
```

## 扩展性设计

### 插件市场支持
- 使用trait定义插件接口，支持WASM或动态库加载
- 插件沙箱隔离，限制权限
- 插件注册表管理已安装插件

### AI功能扩展
- Provider trait抽象，方便切换AI后端
- 支持OpenAI、Claude、本地模型等
- 可扩展的分析和生成功能

### 数据存储扩展
- 使用SQLite作为主要存储
- 支持JSON导出/导入
- 预留云同步接口

## 后端服务扩展（用户系统/云同步）

如果后期需要用户注册登录、云同步等功能，建议新增以下模块：

```
src-tauri/src/
├── ...existing modules...
│
├── api/                 # 远程API调用 ⭐新增
│   ├── mod.rs
│   ├── client.rs        # HTTP客户端封装
│   ├── auth.rs          # 认证相关(登录/注册/Token)
│   └── sync.rs          # 云同步API
│
├── auth/                # 本地认证模块 ⭐新增
│   ├── mod.rs
│   ├── token.rs         # Token管理(存储/刷新)
│   ├── session.rs       # 会话管理
│   └── keychain.rs      # 安全存储(系统钥匙串)
│
└── commands/
    └── auth.rs          # 认证命令 ⭐新增
```

### 认证流程设计

```
┌─────────┐      ┌─────────────┐      ┌─────────────┐
│  前端   │ ──── │  Tauri命令  │ ──── │  远程后端   │
│  Vue    │      │  commands/  │      │  REST API   │
└─────────┘      └─────────────┘      └─────────────┘
     │                  │                    │
     │  invoke('login') │                    │
     │ ───────────────> │                    │
     │                  │  POST /auth/login  │
     │                  │ ─────────────────> │
     │                  │     JWT Token      │
     │                  │ <───────────────── │
     │                  │                    │
     │                  │ 存储到Keychain     │
     │   登录成功       │                    │
     │ <─────────────── │                    │
```

### 云同步架构

```rust
// api/sync.rs 示例
#[async_trait]
pub trait SyncService: Send + Sync {
    async fn push_activities(&self, activities: &[Activity]) -> Result<()>;
    async fn pull_activities(&self, since: DateTime<Utc>) -> Result<Vec<Activity>>;
    async fn sync(&self) -> Result<SyncResult>;
}

// 本地优先策略
pub struct SyncManager {
    local: Box<dyn Storage>,
    remote: Box<dyn SyncService>,
    conflict_resolver: ConflictResolver,
}
```

### 后端API建议结构（独立项目）

如需搭建独立后端服务，建议使用Rust或Node.js：

```
dailycraft-server/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── auth.rs      # /auth/login, /auth/register
│   │   ├── sync.rs      # /sync/push, /sync/pull
│   │   └── user.rs      # /user/profile
│   ├── models/
│   ├── services/
│   └── middleware/
│       └── auth.rs      # JWT验证中间件
└── Cargo.toml
```

## 依赖规划

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
async-trait = "0.1"
# 后端服务扩展依赖
reqwest = { version = "0.11", features = ["json"] }  # HTTP客户端
keyring = "2"                                         # 系统钥匙串
jsonwebtoken = "9"                                    # JWT处理
```
