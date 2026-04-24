#!/usr/bin/env python3
"""
Genere une representation 2D (SVG) du batiment a partir des exports JSON.

Sortie:
  exports/building_2d_from_exports.svg
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[1]
EXPORTS_DIR = ROOT / "exports"
INDEX_FILE = EXPORTS_DIR / "index.json"
OUT_FILE = EXPORTS_DIR / "building_2d_from_exports.svg"


def load_index() -> list[dict]:
    data = json.loads(INDEX_FILE.read_text(encoding="utf-8"))
    levels = data.get("levels", [])
    if not levels:
        raise RuntimeError("Aucun niveau trouve dans exports/index.json")
    return levels


def load_level(json_file: str) -> dict:
    level_path = EXPORTS_DIR / json_file
    return json.loads(level_path.read_text(encoding="utf-8"))


def wall_runs(occupancy: list[int], width: int, height: int) -> Iterable[tuple[int, int, int]]:
    """Retourne des segments horizontaux de murs (row, start_x, run_len)."""
    for row in range(height):
        base = row * width
        col = 0
        while col < width:
            if occupancy[base + col] != 1:
                col += 1
                continue
            start = col
            col += 1
            while col < width and occupancy[base + col] == 1:
                col += 1
            yield row, start, col - start


def main() -> None:
    levels_meta = load_index()

    panel_padding = 16
    title_h = 24
    panel_title_h = 18
    cols = 2

    level_payloads = []
    max_w = 0
    max_h = 0
    for meta in levels_meta:
        payload = load_level(meta["json_file"])
        grid = payload["grid"]
        width = int(grid["width"])
        height = int(grid["height"])
        max_w = max(max_w, width)
        max_h = max(max_h, height)
        level_payloads.append((meta["name"], grid["occupancy"], width, height))

    rows = (len(level_payloads) + cols - 1) // cols
    panel_w = max_w
    panel_h = max_h + panel_title_h

    svg_w = panel_padding + cols * (panel_w + panel_padding)
    svg_h = title_h + panel_padding + rows * (panel_h + panel_padding)

    parts: list[str] = []
    parts.append('<?xml version="1.0" encoding="UTF-8"?>')
    parts.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{svg_w}" height="{svg_h}" '
        f'viewBox="0 0 {svg_w} {svg_h}" shape-rendering="crispEdges">'
    )
    parts.append('<rect x="0" y="0" width="100%" height="100%" fill="#f3f4f6"/>')
    parts.append(
        f'<text x="{panel_padding}" y="{title_h - 6}" font-family="Arial, sans-serif" '
        f'font-size="16" fill="#111827">Representation 2D du batiment (murs detectes)</text>'
    )

    for i, (name, occupancy, width, height) in enumerate(level_payloads):
        col = i % cols
        row = i // cols
        x0 = panel_padding + col * (panel_w + panel_padding)
        y0 = title_h + panel_padding + row * (panel_h + panel_padding)

        parts.append(
            f'<rect x="{x0}" y="{y0}" width="{panel_w}" height="{panel_h}" '
            f'fill="#ffffff" stroke="#9ca3af" stroke-width="1"/>'
        )
        parts.append(
            f'<text x="{x0 + 6}" y="{y0 + 13}" font-family="Arial, sans-serif" '
            f'font-size="12" fill="#111827">{name}</text>'
        )

        grid_x = x0
        grid_y = y0 + panel_title_h

        for y, start_x, run_len in wall_runs(occupancy, width, height):
            parts.append(
                f'<rect x="{grid_x + start_x}" y="{grid_y + y}" width="{run_len}" height="1" fill="#111827"/>'
            )

    parts.append("</svg>")
    OUT_FILE.write_text("\n".join(parts), encoding="utf-8")
    print(f"SVG genere: {OUT_FILE}")


if __name__ == "__main__":
    main()
