report機能実装の部分を進めていくと、コンパイルできなくなる原因として
以下の部分がある

Cargo.tomlでcargo add serdeする前は以下でも良いが、
```
chrono = "0.4.41"
```

addしたら以下のように構造体のシリアライズを有効にする必要がある
```
chrono = { version = "0.4.41", features = ["serde"] }
```