#!/usr/bin/env python3
import argparse
import re
import subprocess
import sys
from pathlib import Path

from fontTools.merge import Merger
from fontTools.pens.cu2quPen import Cu2QuPen
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTCollection, TTFont, newTable
from fontTools.ttLib.scaleUpem import scale_upem

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path("/usr/share/fonts/noto-cjk/NotoSansCJK-Bold.ttc")
UI_FONT = ROOT / "assets" / "RobotoCondensed-Bold.ttf"
LANGUAGE_LABELS = ROOT / "locales" / "en-US" / "settings" / "language.ftl"
LATIN_OUTPUT = "RobotoCondensed-Bold-UI.ttf"

SHARED_RANGES = [
    (0x3000, 0x303F),
    (0x3040, 0x309F),
    (0x30A0, 0x30FF),
    (0x31F0, 0x31FF),
    (0xFF01, 0xFF60),
    (0xFFE0, 0xFFE6),
]

TARGETS = [
    ("zh-CN", 2, "settings-language-chinese", "RobotoCondensed-Bold-SC.ttf"),
    ("ja-JP", 0, "settings-language-japanese", "RobotoCondensed-Bold-JP.ttf"),
]

MAXP_FIELDS = (
    "maxZones",
    "maxTwilightPoints",
    "maxStorage",
    "maxFunctionDefs",
    "maxInstructionDefs",
    "maxStackElements",
    "maxSizeOfInstructions",
    "maxComponentElements",
)


def catalog_codepoints(locale):
    found = set()
    for path in sorted((ROOT / "locales" / locale).rglob("*.ftl")):
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.startswith("#"):
                continue
            text = re.sub(r"^[A-Za-z0-9-]+\s*=", "", line)
            found |= {ord(ch) for ch in text if ord(ch) > 0x2000}
    return found


def baseline_codepoints(locale):
    path = ROOT / "data" / "fonts" / f"{locale}.codepoints"
    return {int(line, 16) for line in path.read_text().split()}


def label_codepoints(key):
    for line in LANGUAGE_LABELS.read_text(encoding="utf-8").splitlines():
        name, _, value = line.partition("=")
        if name.strip() == key:
            return {ord(ch) for ch in value.strip()}
    sys.exit(f"missing {key} in {LANGUAGE_LABELS}")


def subset(full, codepoints, output):
    subprocess.run(
        [
            "pyftsubset",
            str(full),
            f"--unicodes={','.join(f'{cp:04X}' for cp in sorted(codepoints))}",
            f"--output-file={output}",
            "--layout-features=",
            "--no-hinting",
            "--desubroutinize",
        ],
        check=True,
    )
    return output


def quadratic(source, template, output):
    font = TTFont(source)
    order = font.getGlyphOrder()
    glyph_set = font.getGlyphSet()
    glyphs = {}
    for name in order:
        pen = TTGlyphPen(glyph_set)
        glyph_set[name].draw(Cu2QuPen(pen, 1.0, reverse_direction=True))
        glyphs[name] = pen.glyph()
    for tag in list(font.keys()):
        if tag not in template.keys():
            del font[tag]
    font["loca"] = newTable("loca")
    glyf = font["glyf"] = newTable("glyf")
    glyf.glyphOrder = order
    glyf.glyphs = glyphs
    glyf.compile(font)
    for name, glyph in glyphs.items():
        if hasattr(glyph, "xMin"):
            font["hmtx"][name] = (font["hmtx"][name][0], glyph.xMin)
    maxp = font["maxp"] = newTable("maxp")
    maxp.tableVersion = 0x00010000
    for field in MAXP_FIELDS:
        setattr(maxp, field, 1 if field == "maxZones" else 0)
    post = font["post"]
    post.formatType = 2.0
    post.extraNames = []
    post.mapping = {}
    post.glyphOrder = order
    font.sfntVersion = "\x00\x01\x00\x00"
    scale_upem(font, template["head"].unitsPerEm)
    font.save(output)
    return output


def merge(parts, output):
    Merger().merge([str(UI_FONT), *map(str, parts)]).save(output)
    size = output.stat().st_size // 1024
    print(f"{output.name}: {size} KB, {len(TTFont(output).getGlyphOrder())} glyphs")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    args = parser.parse_args()
    if not args.source.is_file():
        sys.exit(f"missing source font {args.source}; install noto-cjk")

    template = TTFont(UI_FONT)
    covered = set(template.getBestCmap())
    shared = {cp for start, end in SHARED_RANGES for cp in range(start, end + 1)}
    collection = TTCollection(args.source)
    work = ROOT / "target" / "font-subset"
    work.mkdir(parents=True, exist_ok=True)

    labels = []
    for locale, face, label, name in TARGETS:
        full = work / f"{locale}-full.otf"
        collection.fonts[face].save(full)
        wanted = (shared | baseline_codepoints(locale) | catalog_codepoints(locale)) - covered
        cjk = subset(full, wanted, work / f"{locale}-subset.otf")
        merge([quadratic(cjk, template, work / f"{locale}.ttf")], ROOT / "assets" / name)
        own = subset(full, label_codepoints(label) - covered, work / f"{locale}-label.otf")
        labels.append(quadratic(own, template, work / f"{locale}-label.ttf"))
    merge(labels, ROOT / "assets" / LATIN_OUTPUT)


if __name__ == "__main__":
    main()
