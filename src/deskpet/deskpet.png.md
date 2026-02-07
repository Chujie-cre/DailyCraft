# 桌宠图格式规范

本文档说明如何为 PawPrint 桌宠制作自定义角色精灵图。

## 图片基本要求

- **格式**: PNG（必须带透明背景 Alpha 通道）
- **存放路径**: `public/pets/角色名.png`
- **帧尺寸**: 每帧固定 **128×128 像素**（可通过 `frameSize` 修改，但全图必须统一）
- **角色朝向**: 默认朝右绘制，代码通过 `scaleX * -1` 自动翻转为朝左

## 桌宠图片布局

```
每一行 = 一个动画状态
每一列 = 该状态的一帧（从左到右排列）

┌────────┐
│ 行1    │ [帧1]                          ← stand（站立）
│ 行2    │ [帧1][帧2][帧3][帧4]           ← walk（行走）
│ 行3    │ [帧1]                          ← sit（坐下）
│ 行4    │ [帧1][帧2]...[帧N]             ← idle（发呆/特殊动作）
│ 行5    │ [帧1]                          ← jump（跳跃）
│ 行6    │ [帧1][帧2][帧3]                ← fall（落地缓冲）
│ 行7    │ [帧1]                          ← drag（被拖拽）
│ 行8    │ [帧1][帧2]...[帧N]             ← crawl（天花板爬行）
│ 行9    │ [帧1][帧2]...[帧N]             ← climb（墙壁攀爬）
└────────┘
```

### 尺寸计算

- **图片总宽度** = `frameSize × 最大帧数行的帧数`（如最多8帧 → 128×8 = 1024px）
- **图片总高度** = `frameSize × 总行数`（如9行 → 128×9 = 1152px）

### 布局示例（Pusheen）

以当前 Pusheen 猫为例，精灵图为 **1024×1152** 像素：

| 行号 | 状态 | 帧数 | 说明 |
|:----:|:----:|:----:|------|
| 1 | stand | 1 | 站立静态 |
| 2 | walk | 4 | 行走循环 |
| 3 | sit | 1 | 坐下静态 |
| 4 | idle | 8 | 吃泡面/想披萨 |
| 5 | jump | 1 | 跳跃姿势 |
| 6 | fall | 3 | 落地翻滚 |
| 7 | drag | 1 | 被拖拽悬空 |
| 8 | crawl | 8 | 天花板横向爬行 |
| 9 | climb | 8 | 墙壁纵向攀爬 |

## 配置文件

修改 `src/deskpet/config/defaultPet.ts`：

```typescript
import { ISpriteConfig } from "../types/ISpriteConfig";

const defaultPetConfig: ISpriteConfig = {
    name: "角色名",               // 唯一标识
    imageSrc: "pets/角色名.png",   // 相对 public/ 的路径
    frameSize: 128,               // 每帧像素尺寸（宽=高）
    states: {
        stand: { spriteLine: 1, frameMax: 1 },   // spriteLine=第几行, frameMax=帧数
        walk:  { spriteLine: 2, frameMax: 4 },
        sit:   { spriteLine: 3, frameMax: 1 },
        idle:  { spriteLine: 4, frameMax: 8 },
        jump:  { spriteLine: 5, frameMax: 1 },
        fall:  { spriteLine: 6, frameMax: 3 },
        drag:  { spriteLine: 7, frameMax: 1 },
        crawl: { spriteLine: 8, frameMax: 8 },
        climb: { spriteLine: 9, frameMax: 8 },
    },
};

export default defaultPetConfig;
```

## 状态说明

### 必需状态

| 状态 | 说明 | 建议帧数 |
|------|------|:--------:|
| `stand` | 站立/默认姿势 | 1-4 |
| `walk` | 水平行走循环 | 2-8 |
| `drag` | 被鼠标拖拽时的悬空姿势 | 1-2 |

### 推荐状态

| 状态 | 说明 | 建议帧数 |
|------|------|:--------:|
| `jump` | 被甩出时的跳跃姿势 | 1-2 |
| `fall` | 落地后的缓冲动画（播放一次） | 2-4 |

### 可选状态

| 状态 | 说明 | 建议帧数 |
|------|------|:--------:|
| `climb` | 贴墙攀爬（碰到左右边界时触发） | 2-8 |
| `crawl` | 天花板爬行（碰到顶部后触发） | 2-8 |
| `sit` | 随机坐下休息 | 1-4 |
| `idle` | 发呆/吃东西等特殊动画 | 2-8 |

> 没有的状态直接不写在 `states` 里，代码会自动跳过对应行为。

## 注意事项

1. **行号可以跳跃** — `spriteLine` 指定实际使用精灵图的第几行，中间可以留空行
2. **每行帧数独立** — 不同行可以有不同的帧数，通过 `frameMax` 指定
3. **帧从最左侧开始** — 每行的帧必须从第1列开始，不能有左侧空白偏移
4. **角色居中绘制** — 每个 128×128 帧内，角色应尽量居中放置
5. **climb 和 crawl 方向** — climb 画侧面攀爬姿势，crawl 画底视角爬行姿势
