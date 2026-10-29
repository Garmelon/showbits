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
for i, event in enumerate(data["results"]):
    date = datetime.datetime.fromisoformat(event["date"])
    date = date.astimezone()
    date = date.strftime("%Y-%m-%d %H:%M %Z")

    events.append(
        {
            "name": event["name"],
            "description": event["description"],
            "location": event["location"],
            "date": date,
            "date_precision": event["date_precision"]["name"],
        }
    )

EVENTS.write_text(json.dumps(events))
