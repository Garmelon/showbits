#!/usr/bin/env python3

import argparse
import json
from pathlib import Path
from subprocess import run

DIR = Path(__file__).parent
INFO = DIR / "info.json"
IMAGE = DIR / "image.png"


class Args:
    number: int | None


parser = argparse.ArgumentParser()
parser.add_argument("number", type=int, nargs="?")
args = parser.parse_args(namespace=Args())

url = "https://xkcd.com/info.0.json"
if args.number is not None:
    url = f"https://xkcd.com/{args.number}/info.0.json"

info = json.loads(
    run(
        ["curl", url],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
)

run(["curl", "-o", IMAGE, info["img"]], check=True)

size = json.loads(
    run(
        ["magick", "identify", "-format", '{"w":%w,"h":%h}', IMAGE],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
)

INFO.write_text(
    json.dumps(
        {
            "num": info["num"],
            "title": info["title"],
            "alt": info["alt"],
            "width": size["w"],
            "height": size["h"],
        }
    )
)
