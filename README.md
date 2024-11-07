# tetris

## 游戏介绍

俄罗斯方块是一款经典的益智游戏，玩家需要通过旋转和移动各种形状的方块，使其在底部堆积成完整的行。每完成一行，行就会消失并得分。游戏的目标是尽可能长时间地避免方块堆积到屏幕顶部。

## 基本玩法

玩家可以通过以下按键来控制方块：

- `d` 或 `→`：向右移动方块
- `q` 或 `←`：向左移动方块
- `z` 或 `↑`：顺时针旋转方块
- `c`：顺时针旋转方块
- `x`：逆时针旋转方块
- `s`：快速下降方块
- `h`：暂存或释放当前方块
- `空格`：快速下落方块直到碰到底部

通过这些操作，玩家可以调整方块的位置和方向，使其在底部堆积成完整的行。

## 构建方式

### 安装依赖

在开始构建项目之前，请确保你已经安装了以下工具：

- Rust（Cargo）
- wasm-pack
- python

### 构建步骤
1. 克隆项目到本地：

```bash
git clone https://github.com/Serein207/tetris.git
```

2. 构建运行项目（本机窗口应用程序）

```bash
cargo build --release
cargo run
```

3. 构建项目（WebAssembly）

```bash
wasm-pack build --release --target web
python3 -m http.server
```

访问 http://localhost:8000/ 即可开始游戏。

## 项目依赖

- slint
- rand
- wasm-pack
