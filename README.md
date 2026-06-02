# Reminder

轻量级跨平台桌面专注提醒（Reminder），基于 **Tauri 2 + SvelteKit** 构建。支持中英文界面切换。

## 界面预览

### 专注计时

主界面圆形进度环倒计时，显示当前专注状态与剩余时间。

![专注计时主界面](screenshots/focus-timer.png)

### 设置 · 时长

可配置工作时长、短休与长休，支持预设档位与自定义输入。

![设置 - 时长](screenshots/settings-duration.png)

### 设置 · 偏好

每日专注目标、休息背景、主题色与开机自启等个性化选项。

![设置 - 偏好](screenshots/settings-preferences.png)

## 给玩家（最终用户）

**安装 exe 后可直接使用，无需安装 Rust、Node.js 或任何开发工具。**

安装包已内置：
- 应用本体（Rust 已编译进 exe，运行时不需要 Rust）
- WebView2 引导程序（电脑没有 WebView2 时会自动安装）

## 给开发者（打包安装包）

### 一键打包（推荐）

双击运行：

```
scripts\build-installer.bat
```

脚本会自动：
1. 检测并安装 Rust（仅本机打包需要）
2. 安装 npm 依赖
3. 编译并生成 NSIS 安装包

### 或使用 npm 命令

```powershell
cd Reminder
npm install
npm run tauri:build    # 自动检查/安装 Rust 后打包
```

### 开发调试

```powershell
npm run tauri:dev      # 自动检查/安装 Rust 后启动
```

### 安装包输出位置

```
src-tauri\target\release\bundle\nsis\
```

将生成的 `.exe` 安装程序分发给玩家即可。

## 功能

- 专注计时（圆形进度环）
- 工作结束休息提醒弹窗
- 全屏休息模式（支持自定义背景图、长按 6 秒结束）
- 设置：工作/短休/长休时长、长休模式、严格休息、自动休息、每日目标
- 自定义主题色、开机自启
- 系统托盘

## 项目结构

```
Reminder/
├── screenshots/              # 界面截图
├── scripts/
│   ├── setup-rust.ps1        # 自动安装 Rust（仅开发机）
│   └── build-installer.bat   # 一键打包
├── src/                      # SvelteKit 前端
├── src-tauri/                # Rust 后端
└── static/
```

## 说明

| 角色 | 是否需要 Rust |
|------|--------------|
| 开发者打包 | 是（脚本可自动安装） |
| 玩家使用 | **否** |

- 主窗口：380×560，无边框，可拖拽标题栏
- 休息提醒：屏幕右下角小窗
- 全屏休息：独立全屏窗口
- 配置保存在 `%APPDATA%\com.reminder.app\config.json`
