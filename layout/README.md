# 说明

`Layout` proc_macro 用于为结构体生成各字段按 packed 表示在内存中的布局范围。如：

```rust
#[derive(Layout)]
struct A {
    a: u32;
    b: u8;
}
```

会生成：

```rust
struct ALayout;
impl ALayout {
    pub const fn a() -> core::ops::Range<usize> {
        0..4 // 0, 1, 2, 3 号字节
    }
    pub const fn b() -> core::ops::Range<usize> {
        4..5 // 4 号字节
    }
}
```