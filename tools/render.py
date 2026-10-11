#!/usr/bin/env python3
"""動画からモノクロ ASCII フレームを生成する (開発用)。

`assets/frames.txt` を作る。ffmpeg と Python 3 が必要。

  tools/render.py <動画> --fps 12 --cols 34 --rows 20

ffmpeg は PATH か環境変数 FFMPEG から探す。
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def find_ffmpeg() -> str:
    ff = os.environ.get("FFMPEG") or shutil.which("ffmpeg")
    if not ff:
        sys.exit("ffmpeg が見つかりません (FFMPEG 環境変数か PATH で指定してください)")
    return ff


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("video", nargs="?", default=os.environ.get("CLPCAT_SOURCE"), help="入力動画")
    ap.add_argument("--fps", type=int, default=12)
    ap.add_argument("--cols", type=int, default=34)
    ap.add_argument("--rows", type=int, default=20)
    ap.add_argument("--crop", default="290:340:35:100", help="ffmpeg crop=w:h:x:y")
    ap.add_argument("--contrast", type=float, default=2.0)
    ap.add_argument("--brightness", type=float, default=0.42)
    ap.add_argument("--ramp", default="@%#*+=-:. ", help="暗→明の順の文字ランプ")
    ap.add_argument("--out", default=str(ROOT / "assets" / "frames.txt"))
    args = ap.parse_args()

    if not args.video:
        ap.error("動画パスを指定してください (または環境変数 CLPCAT_SOURCE を設定)")

    ff = find_ffmpeg()
    ramp = args.ramp
    nlast = len(ramp) - 1

    vf = f"fps={args.fps},crop={args.crop},format=gray,scale={args.cols}:{args.rows}:flags=area"
    raw = subprocess.check_output(
        [ff, "-y", "-loglevel", "error", "-i", args.video,
         "-vf", vf, "-pix_fmt", "gray", "-f", "rawvideo", "-"]
    )

    cw = args.cols * args.rows
    nframes = len(raw) // cw
    if nframes == 0:
        sys.exit("フレームを生成できませんでした (動画パスを確認してください)")

    def map_value(v: int) -> str:
        v2 = args.contrast * (v - 128) + 128 + args.brightness * 255
        v2 = 0.0 if v2 < 0 else 255.0 if v2 > 255 else v2
        return ramp[int(v2 * nlast / 255)]

    frames_out: list[str] = []
    for fi in range(nframes):
        base = fi * cw
        lines: list[str] = []
        for y in range(args.rows):
            row = raw[base + y * args.cols: base + y * args.cols + args.cols]
            lines.append("".join(map_value(v) for v in row).rstrip())
        frames_out.append("\n".join(lines))

    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\x0c".join(frames_out) + "\n")
    print(f"{nframes} フレーム → {out} ({out.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
