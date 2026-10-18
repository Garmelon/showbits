import argparse
import shlex
import subprocess
import time
from pathlib import Path

import cv2

YOUTUBE_URL = "https://www.youtube.com/watch?v=FtutLA63Cp8"
WINDOW_TITLE = "last capture"
DISPLAY_DELAY = 30

DIR = Path(__file__).parent
SRC_VIDEO = DIR / "bad_apple_src.mp4"
SRC_FRAMES = DIR / "frames_src"
INK_VIDEO = DIR / "bad_apple_ink.mp4"
INK_FRAMES = DIR / "frames_ink"

parser = argparse.ArgumentParser()
parser.add_argument("address")
args = parser.parse_args()


def run(*args: str | Path) -> None:
    print(f"$ {' '.join(shlex.quote(str(arg)) for arg in args)}")
    subprocess.run(args, check=True)


DIR.mkdir(parents=True, exist_ok=True)
SRC_FRAMES.mkdir(parents=True, exist_ok=True)
INK_FRAMES.mkdir(parents=True, exist_ok=True)

if not SRC_VIDEO.exists():
    run("yt-dlp", YOUTUBE_URL, "-t", "mp4", "-o", SRC_VIDEO.with_suffix(".%(ext)s"))

run("ffmpeg", "-i", SRC_VIDEO, SRC_FRAMES / "%04d.png")

cv2.namedWindow(WINDOW_TITLE)
cam = cv2.VideoCapture(0)

frames = sorted(SRC_FRAMES.iterdir())
for i, frame in enumerate(frames):
    print(f"On frame {i + 1:04}/{len(frames)}")

    run(DIR / "show_frame", args.address, frame)
    start = time.time()

    while time.time() < start + DISPLAY_DELAY:
        ret, capture = cam.read()
        cv2.imshow(WINDOW_TITLE, capture)
        cv2.waitKey(20)

    cv2.imwrite(INK_FRAMES / frame.name, capture)

cam.release()
cv2.destroyWindow(WINDOW_TITLE)

run("ffmpeg", "-i", INK_FRAMES / "%04d.png", INK_VIDEO)
