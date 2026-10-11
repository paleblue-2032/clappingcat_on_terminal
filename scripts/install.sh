#!/usr/bin/env bash
#
# clpcat インストーラ (Linux)
#
#   curl -fsSL https://raw.githubusercontent.com/paleblue-2032/clappingcat_on_terminal/main/scripts/install.sh | bash
#
# 環境変数:
#   CLPCAT_VERSION … 入れるバージョン (既定: latest)
#   CLPCAT_PREFIX  … インストール先の prefix (既定: ~/.local)
set -euo pipefail

REPO="paleblue-2032/clappingcat_on_terminal"
VERSION="${CLPCAT_VERSION:-latest}"
PREFIX="${CLPCAT_PREFIX:-$HOME/.local}"

say() { printf '%s\n' "$*"; }
die() { printf 'error: %s\n' "$*" >&2; exit 1; }

os="$(uname -s)"
arch="$(uname -m)"

[ "$os" = "Linux" ] || die "このスクリプトは Linux 用です (Nix なら: nix run github:$REPO)"

case "$arch" in
  x86_64 | amd64) asset="clpcat-x86_64.tar.gz" ;;
  aarch64 | arm64) asset="clpcat-aarch64.tar.gz" ;;
  *) die "未対応のアーキテクチャ: $arch (Nix なら: nix run github:$REPO)" ;;
esac

if [ "$VERSION" = "latest" ]; then
  url="https://github.com/$REPO/releases/latest/download/$asset"
else
  url="https://github.com/$REPO/releases/download/v${VERSION#v}/$asset"
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

say "clpcat をダウンロードします: $asset"
if command -v curl >/dev/null 2>&1; then
  curl -fL --retry 3 -o "$tmp/$asset" "$url"
elif command -v wget >/dev/null 2>&1; then
  wget -O "$tmp/$asset" "$url"
else
  die "curl か wget が必要です"
fi

tar -xzf "$tmp/$asset" -C "$tmp"
mkdir -p "$PREFIX/bin"
install -m 0755 "$tmp/clpcat-${arch}/clpcat" "$PREFIX/bin/clpcat"

say "インストールしました: $PREFIX/bin/clpcat"

case ":$PATH:" in
  *":$PREFIX/bin:"*) ;;
  *) say "ヒント: PATH に $PREFIX/bin を追加すると 'clpcat' で実行できます" ;;
esac

say ""
say "タイポで煽らせるには、シェルの設定に以下を追加:"
say "  # zsh (~/.zshrc)"
say "  command_not_found_handler() { print -u2 \"zsh: command not found: \$1\"; clpcat; return 127; }"
