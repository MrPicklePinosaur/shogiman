<div align="center">

# 将棋マン // shogiman

将棋できるプログラム

[![crates.io](https://img.shields.io/crates/v/shogiman.svg)](https://crates.io/crates/shogiman)
[![docs.rs](https://docs.rs/shogiman/badge.svg)](https://docs.rs/shogiman)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

## 開発環境のセットアップ SETUP

作用されるのツール
- `lld`や`mold`: fast buildsのため
- `clang`

*注意*: moldリンカーのほうが早いですが、おさらくwasmへのビルドをサポートしていないです。何か問題がある場合はかならず`lld`やrust linkerを使用してください（詳細は[こちら](https://bevyengine.org/learn/book/getting-started/setup/)）

まず開発者向けにgitフックをインストールします
```sh
just devsetup
```

実行したい場合は
```sh
just
```

## 研究　RESEARCH

- [USI: Universal Shogi Inteface](http://shogidokoro.starfree.jp/usi.html)

## 語彙　TERMINOLOGY

先手（せんて）
後手（ごて）

SFEN(Shogi Forsyth-Edwards Notation)

### 駒

歩兵（ふひょう）- 歩
香車（きょうしゃ）- 香
桂馬（けいま）- 桂
銀将（ぎんしょう）- 銀
金将（きんしょう）- 金
角行（かくぎょう）- 角
飛車（ひしゃ）- 飛
王将（おうしょう）

### 成った駒

と金（ときん）
成香（なりきょう）
成桂（なりけい）
成銀（なりぎん）
龍馬（りゅうま）
竜王（りゅうおう）

