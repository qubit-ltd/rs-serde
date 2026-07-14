# Duration Unit Serde Adapter Design

## 背景

现有 `duration_with_unit` 在序列化时始终把 `Duration` 四舍五入为整毫秒，
但名称没有表达固定毫秒和有损语义。它还通过 `serde_json::Value` 完成反序列化，
因此依赖 `deserialize_any`，无法与 postcard 等非自描述格式稳定 round-trip。

下游使用场景分成两类：

- `rs-config` 与 `rs-retry` 的文本展示需要稳定的毫秒输出；
- `rs-progress` 的 elapsed 是状态数据，需要保留 `Instant` 提供的亚毫秒精度。

## 目标

- 将现有固定毫秒字符串适配器命名为 `duration_millis_with_unit`。
- 提供精确的 `duration_with_unit`，自动选择规范且无精度损失的单位。
- 使用自定义 Serde Visitor 支持 human-readable 宽松输入和非
  human-readable 字符串协议。
- 让直接调用 `parse` 的用户获得稳定、结构化的 `ParseDurationError`。
- 用边界、极值、协议和下游测试固定语义。

## 非目标

- 不提供运行时或属性参数化的“指定输出单位”接口。
- 不移除 `qubit-datatype`，也不重构其依赖图。
- 不改变整数适配器 `duration_millis` 的 wire format。
- 不执行 Git 提交或推送。

## 公共 API

### `serde::duration_millis_with_unit`

- `serialize` 和 `format` 始终输出四舍五入后的 `<millis>ms`。
- `deserialize` 接受显式单位字符串；human-readable 格式还接受非负毫秒整数。
- `parse` 接受 `[0-9]+(ns|us|ms|s|m|h|d)?`，裸数字按毫秒解释。
- 该模块明确标注为有损格式，适用于展示和兼容配置。

### `serde::duration_with_unit`

- `serialize` 和 `format` 精确保留 `Duration`。
- 从 `d`、`h`、`m`、`s`、`ms`、`us`、`ns` 中选择能够整除总纳秒数的最大单位。
- 零值固定输出 `0ms`；其他输出示例包括 `2m`、`2500ms`、`500us`、`42ns`。
- `parse(format(value)) == value` 对全部 `Duration` 成立，包括 `Duration::MAX`。
- 输入协议与 `duration_millis_with_unit` 相同。

不增加任意单位参数，因为 `#[serde(with = "...")]` 无法传递该参数，现有下游也没有
此需求。未来若出现手写 `Serialize` 或 `serde_with` 场景，再独立设计。

## 反序列化协议

共享 Visitor 根据 `Deserializer::is_human_readable()` 选择入口：

- human-readable：调用 `deserialize_any`，接受字符串或非负整数；
- non-human-readable：调用 `deserialize_str`，与序列化端固定的字符串 wire format
  对称。

这样既保留 JSON 配置对整数毫秒的兼容，也让 postcard 等格式不再依赖
`deserialize_any`。

## 错误模型

`ParseDurationError` 是 `#[non_exhaustive]` 的公开枚举：

- `InvalidSyntax`：不符合规范语法；
- `UnsupportedUnit { unit: String }`：后缀语法合法但单位未知；
- `OutOfRange`：数值超过 `u128` 或不能表示为 `Duration`。

错误类型不暴露 `DataConversionError`，避免把底层转换实现固化为公共契约。

## 下游迁移

- `rs-config`：文本转换改用 `duration_millis_with_unit::format`。
- `rs-retry`：`RetryDelayDurationFormat` 改用
  `duration_millis_with_unit::{format, parse}`。
- `rs-progress`：继续使用 `duration_with_unit`，但获得新的精确语义。

## 验证

- 固定毫秒适配器覆盖 499µs、500µs、1499µs、1500µs 舍入边界。
- 精确适配器覆盖全部单位、零值、亚毫秒值和 `Duration::MAX` round-trip。
- 直接断言 `ParseDurationError` 变体和单位内容。
- JSON 覆盖字符串、整数和错误类型；postcard 覆盖非 human-readable round-trip。
- 分别运行 `rs-serde`、`rs-progress`、`rs-retry`、`rs-config` 的测试、fmt、clippy
  和 rustdoc 检查。
