# clpcat

**タイポしたら黒猫が拍手して煽ってくる**、ターミナル用の小さなCLI。

存在しないコマンドを打つと、画面いっぱいに黒猫が現れてパチパチしながら煽ります。
コマンド名は `clpcat`。

```
           =.       =#=
           *%*-:::=###:
           :%%##*#####
            ##*###**##
            =####**###
             ###+=*#*#=.
            =%%%%%###%%#=
          ..%%%%%%%##*#%#+
          .+#@@@%%%###%%%#
        -##*#@@@@@@%%%%%%*
        -###%@@@@@@@@@%%%+
        .--#%@@@@@@@@%%%%=
           +%%@@@%%@%%%%%-
            =%%%%%@@%%%%%.
            .%%%%%@%%%%%#
             *%%#%%%%%%%-
             ++.   -*%%=
```

## 特徴

- **モノクロ記号ASCIIでアニメーション** — 元動画を1フレームずつ文字に落とし込み、そのままターミナルで再生
- **タイポを検知** — zsh / bash の `command_not_found` フックから呼ぶと、打ち間違いのたびに猫が踊る
- **依存が少ない** — Rust製の単体バイナリ。`q` / `Esc` / `Ctrl-C` でいつでも閉じられる

## 使い方

```sh
clpcat                # 猫が1回踊る
clpcat -n 3           # 3回繰り返す
clpcat -n 0           # 無限ループ（q で閉じる）
clpcat --fps 24       # 再生速度を上げる
```

| オプション | 説明 | 既定 |
| --- | --- | --- |
| `-n, --repeat <N>` | 再生回数（`0` で無限ループ） | `1` |
| `--fps <FPS>` | 再生速度（fps） | `12` |

再生中は `q` / `Esc` / `Ctrl-C` で閉じられます。

## インストール

### 1. Nix（NixOS / Nix）

ワンオフ実行:

```sh
nix run github:paleblue-2032/clappingcat_on_terminal
```

プロファイルへ導入:

```sh
nix profile install github:paleblue-2032/clappingcat_on_terminal
```

flake input として（`flake.nix`）:

```nix
{
  inputs.clpcat.url = "github:paleblue-2032/clappingcat_on_terminal";
}
```

home-manager で入れる場合:

```nix
{ inputs, pkgs, ... }:
{
  home.packages = [ inputs.clpcat.packages.${pkgs.system}.clpcat ];
}
```

### 2. プリビルドバイナリ（Linux / musl 静的リンク）

どのディストロでも動く静的バイナリを GitHub Releases で配布しています。

ワンライナー:

```sh
curl -fsSL https://raw.githubusercontent.com/paleblue-2032/clappingcat_on_terminal/main/scripts/install.sh | bash
```

手動で入れる場合（x86_64 の例）:

```sh
curl -fLO https://github.com/paleblue-2032/clappingcat_on_terminal/releases/latest/download/clpcat-x86_64.tar.gz
tar -xzf clpcat-x86_64.tar.gz
install -Dm755 clpcat-x86_64/clpcat ~/.local/bin/clpcat
```

- `x86_64` / `aarch64` の2種類。x86_64 は musl 静的リンクなので glibc のバージョンを気にせず、Alpine でも動きます（aarch64 は glibc ビルド）
- インストール先の prefix は `CLPCAT_PREFIX`、バージョンは `CLPCAT_VERSION` で変更できます

### 3. ソースから

```sh
cargo install --git https://github.com/paleblue-2032/clappingcat_on_terminal
```

## タイポで煽らせる

シェルの「コマンドが見つからない」フックに `clpcat` を仕込みます。

### zsh

`~/.zshrc` に追加:

```zsh
command_not_found_handler() {
  print -u2 "zsh: command not found: $1"
  if [[ -t 1 && -z "$CLPCAT_DISABLE" ]]; then
    clpcat
  fi
  return 127
}
```

### bash

`~/.bashrc` に追加:

```bash
command_not_found_handle() {
  echo "bash: $1: command not found" >&2
  if [ -t 1 ] && [ -z "$CLPCAT_DISABLE" ]; then
    clpcat
  fi
  return 127
}
```

`CLPCAT_DISABLE` を立てておくと自動再生を止められます（`export CLPCAT_DISABLE=1`）。

> NixOS + home-manager なら `programs.zsh.initContent` に上記を書いておくと宣言的に管理できます。

## フレームを作り直す

`assets/frames.txt` は元動画（`Clapping Cat Meme`）から生成したASCIIフレームです。
別の動画に差し替えたいときは `tools/render.py` を使います（`ffmpeg` と Python 3 が必要）:

```sh
tools/render.py path/to/video.mov --fps 12 --cols 34 --rows 20
cargo build --release
```

主な調整ポイント:

- `--crop W:H:X:Y` … 被写体に寄せる（元動画は縦長なので中央の猫を切り出しています）
- `--contrast` / `--brightness` … 背景を空白に飛ばし、被写体を濃く出す
- `--ramp` … 暗→明の文字ランプ（既定 `@%#*+=-:. `）

## 仕組み

- 動画 → `ffmpeg` で fps / クロップ / グレースケール化 → Python で輝度をASCIIランプへ写像 → `assets/frames.txt`
- `assets/frames.txt` はバイナリに `include_str!` で埋め込み（実行時に動画もffmpegも不要）
- `crossterm` でオルタネートスクリーンに中央寄せしてフレームを描画

## ライセンス

[MIT](LICENSE) © 2026 paleblue-2032
