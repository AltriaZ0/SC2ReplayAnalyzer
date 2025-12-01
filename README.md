# SC2ReplayAnalyzer

A analyze Tool for starcraft2 replay ,  based of sc2reader.

Communicate with me: *<zanjune@163.com>*

## Download

下载 SC2ReplayAnalyzer 1.5.0: <https://pan.baidu.com/s/1naOzS-AuCNVyfJ21VSNTpA?pwd=sc2a>

提取码(Password)：sc2a

(环境已经整合，无需配置)

(The environment is already integrated, no configuration needed.)

## 使用教学视频

<https://www.bilibili.com/video/BV135UvB6EhS/?vd_source=bcb1fe379a37616c934618bf7c73570b>

## 🛠️ Build from Source / 源码构建指南

如果您想参与贡献或在本地构建项目，请遵循以下步骤。

### ⚠️ 环境准备与重要前提

在开始之前，请确保您的系统满足以下要求：

1. **🚫 路径无中文**：
    * 项目的存放路径**不能包含中文或空格**，否则 Rust 编译步骤 (`link.exe`) 会直接失败。
    * ❌ 错误示例：`D:\个人文件\SC2ReplayAnalyzer`
    * ✅ 正确示例：`D:\Projects\SC2ReplayAnalyzer`
2. **Rust**: 请访问 [rustup.rs](https://rustup.rs) 下载并安装。
3. **C++ 生成工具**: Windows 用户必须安装 **Visual Studio C++ Build Tools** (安装时需勾选 "使用 C++ 的桌面开发" / "Desktop development with C++")。
4. **Python**: 版本需 **3.10** 或更高。
5. **Node.js**: 建议安装 LTS 版本 (v20+)。

---

### 📥 第一步：获取代码

打开终端（Terminal 或 PowerShell），运行以下命令将项目克隆到本地：

```bash
git clone [https://github.com/AltriaZ0/SC2ReplayAnalyzer.git](https://github.com/AltriaZ0/SC2ReplayAnalyzer.git)
cd SC2ReplayAnalyzer
```

---

### 🖥️ 模式一：命令行调试 (CLI)

CLI 模式主要用于开发和逻辑测试，它会自动读取根目录下 rep/ 文件夹中的 .sc2replay 录像文件进行分析。

#### 准备录像文件:在项目根目录下创建一个名为 rep 的文件夹，并将待分析的录像文件放入其中。

#### 配置 Python 环境:

```bash
# 1. 创建虚拟环境
python -m venv .venv

# 2. 激活虚拟环境 (Windows PowerShell)
.\.venv\Scripts\Activate.ps1

# 3. 安装项目依赖
pip install -e .[build]
```

#### 运行测试

```bash
python src/app/main.py
```

---

### 🎨 模式二：图形界面构建 (GUI / Tauri)

#### 安装前端依赖

```bash
cd front
npm install
```

> 注意：本项目默认推荐使用 npm。如果您习惯使用 pnpm，可能需要修改 tauri.conf.json 配置。

#### 打包 Python 后端 (PyInstaller)

我们需要将 Python 脚本打包成可执行文件，以便 Tauri 能够调用它。
请确保您已激活了 .venv 虚拟环境。

A. 执行打包命令
请在项目根目录下运行以下命令：

```bash
pyinstaller -F -w src\app\main.py --add-data ".venv\Lib\site-packages\sc2reader\data;sc2reader\data"
```

B. 移动并重命名

找到生成的文件：dist/main.exe

确保目标文件夹存在：front/src-tauri/bin/ (没有则新建),将文件移动到目标文件夹.

#### 启动开发界面

回到 front 目录，启动开发服务器：

```bash
npm run tauri dev
```

等待终端显示 "Compiling..."，编译完成后图形界面将自动弹出。

## 更新历史

### 1.0.1更新内容

引入了pandas库

支持导出为excel格式

修复了重新归类变异时，升级列表的长度不足的问题

### 1.1更新内容

增加了对泰伦、普陀寺的支持

增加了个别时间点的人口与农民数量

### 1.1.1更新内容

增加了关键时间点的存活单位和农民数量

### 1.1.2更新内容

增加了进度条，支持显示关键时间点的存活单位

txt文件和excel文件会生成在SC2ReplayAnalyzer文件夹中

修复了之前版本的bug

优化了输出txt和excel内容的格式

### 1.2.0更新内容

修复了之前版本的bug

1.当没有单位变异发生时报错的bug

2.Unitposition出现异常情况的bug

3.变异单位重分类时单位出生的列表可能不够长的bug

目前已经支持批量分析，并支持导出批量rep的流程

由于bug2,修改了sc2reader的一些配置，有可能未来会出现未知的bug

优化了代码架构，使代码可读性增加

优化了GUI，使用更加方便

优化了excel的表现：减去了现在升级科技的*n效果

### 1.2.1更新内容

修复了之前版本的bug：

1.神族能够折跃的单位不会统计单位存活的问题

2.神族VB科技和VR科技名称互换的问题

3.虫族存活单位中变异生成的单位统计不正确的问题

### 1.3.0更新内容

修复了之前版本的bug:

神族合成白球不会减少单位和记录事件不准确的bug

增加了练习功能demo

增加了悬浮文字小插件demo版本

### 1.3.1更新内容

修复了练习功能中时间不准确的问题

### 1.3.2 更新内容

修复了无法读取韩语ID的Rep的bug

修复了秒退的对局分析报错的bug

修复了开局星空加速产农民的bug(会导致农民时间不准确)

修复了多人游戏、人机游戏无法分析的bug

增加了异常报告，现在遇到分析错误的replay不会中止分析而是会跳过并生成异常报告

优化了练习功能的显示效果

提供了分析选择：是否提供所有时间的信息

### 1.3.3 更新内容

发布源码至github

提高了代码的耦合性

修复了练习功能中的若干问题

### 1.4.1 更新内容

1.搭建tauri框架，连接前端和逻辑层
2.重构代码结构，初步实现日志和配置设置
3.重构主分析函数

### 1.4.2

继续重构主分析函数，进行了必要的类封装

### 1.4.3 & 1.4.4

继续完善UI

### 1.5.0

实现了UI页面
实现了简单的分析配置
实现了悬浮文字小插件的tauri实现
软件更名为SC2ReplayAnalyzer

### 1.5.1

暂时移除了悬浮文字的闪烁功能
暂时移除的尚未完善的人口统计功能
更新建造时间到最新版本

### 1.5.2

优化悬浮文字，添加了拖动和锁定的功能，生成的窗口尺寸进行了动态设置
修复了部分toml修改不完全导致的Bug

## 截至目前更新，已经发现的bug

P使用星空加速加速生产/科技升级后，会导致分析的生产/升级时间不准确
当前程序如果分析老版本replay，如果老版本的单位的生产时间进行过调整，会导致分析的生产时间不准确
