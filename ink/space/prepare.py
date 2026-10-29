#!/usr/bin/env python3

import argparse
import datetime
import json
from pathlib import Path
from subprocess import run

URL = "https://lldev.thespacedevs.com/2.3.0/events/upcoming/"

DIR = Path(__file__).parent
EVENTS = DIR / "events.json"


parser = argparse.ArgumentParser()
args = parser.parse_args()

data = json.loads(
    run(
        ["curl", URL],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
)

events = []
for i, event in enumerate(data["results"][:4]):
    image_url: str = event["image"]["thumbnail_url"]
    image_ext = Path(image_url).suffix
    image = DIR / f"image_{i}{image_ext}"
    run(["curl", "-o", image, event["image"]["thumbnail_url"]], check=True)

    date = datetime.datetime.fromisoformat(event["date"])
    date = date.astimezone()
    match event["date_precision"]["id"]:
        case 5:  # Day
            date = date.strftime("%Y-%m-%d")
        case 7:  # Month
            date = date.strftime("%Y-%m")
        case 14:  # Year
            date = date.strftime("%Y")

    events.append(
        {
            "name": event["name"],
            "description": event["description"],
            "location": event["location"],
            "date": date,
            "image": image.name,
        }
    )

EVENTS.write_text(json.dumps(events))
