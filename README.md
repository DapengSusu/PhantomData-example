# PhantomData-example

### Rust 幽灵类型 PhantomData 简单示例
区分普通用户和付费用户的例子

### PhantomData
幽灵类型：广泛用在处理，数据结构定义过程中不需要，但是在实现过程中需要的泛型参数。

可以在编译期做状态的检测，避免运行期检测的负担和潜在的错误。

``` Rust
pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>, // 幽灵类型：广泛用在处理，数据结构定义过程中不需要，但是在实现过程中需要的泛型参数
}
```

### build & run
``` bash
cargo b
cargo test -- --nocapture
```
