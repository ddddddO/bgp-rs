# bgp-rs
「作って学ぶルーティングプロトコル」を元にイベント駆動ステートマシンとしてBGPを実装

## Ref
- [8.  BGP Finite State Machine (FSM)](https://www.rfc-editor.org/rfc/rfc4271#section-8)

## Memo
### cargo test
```console
01:11:37 > rustc --version
rustc 1.67.1 (d5a82bbd2 2023-02-07)
```
だと、

```console
01:04:40 > cargo test
   Compiling bgp-rs v0.1.0 (/home/ochi/github.com/ddddddO/bgp-rs)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:1
  |
1 | #![feature(backtrace, exclusive_range_pattern, arc_unwrap_or_clone)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:12
  |
1 | #![feature(backtrace, exclusive_range_pattern, arc_unwrap_or_clone)]
  |            ^^^^^^^^^
```

な感じで怒られる。

```console
rustup override set nightly
```
で、
```console
01:12:21 > rustc --version
rustc 1.69.0-nightly (7281249a1 2023-02-27)
```

に変更した後、cargo test が通る。
