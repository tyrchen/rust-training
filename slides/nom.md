---
marp: true
theme: uncover
---

<style>
.row {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  width: 100%;
}

.column {
  display: flex;
  flex-direction: column;
  flex-basis: 100%;
  flex: 1;
}
</style>

# Parser Combinator: nom

<style scoped>
    ul { columns: 2; width: 90%; }
</style>

- [基本 Trait](#2)

---

## 基本 Trait
```rust
trait Parser<Input, Output, Error> {
    fn parse(&mut self, input: Input) -> IResult<Input, Output, Error>;
    ...
}
enum Err<E> {
    Incomplete(Needed),
    Error(E),
    Failure(E)
}
```

- Parser Trait (所有 nom parser 都实现了 parser trait)
  - IResult: parser 处理后的结果
  - 自动实现了：`F: FnMut(Input) -> IResult<Input, Output, Error>`
- IResult: `type IResult<I, O, E> = Result<(I, O), Err(E)>;


---

- normal parser
- streaming parser
