import argparse
import shlex
import shutil
import subprocess
import time
from pathlib import Path

import cv2

WINDOW_TITLE = "camera"
DISPLAY_DELAY = 24

parser = argparse.ArgumentParser()
parser.add_argument("address")
parser.add_argument("name")
parser.add_argument("url")
parser.add_argument("-w", "--webcam", type=int, default=0)
args = parser.parse_args()

DIR = Path(__file__).parent
TMP_DIR = DIR / "tmp" / args.name
SRC_VIDEO = TMP_DIR / "src.mp4"
SRC_FRAMES = TMP_DIR / "src"
INK_VIDEO = TMP_DIR / "ink.mp4"
INK_FRAMES = TMP_DIR / "ink"


def run(*args: str | Path) -> None:
    print(f"$ {' '.join(shlex.quote(str(arg)) for arg in args)}")
    subprocess.run(args, check=True)


TMP_DIR.mkdir(parents=True, exist_ok=True)
SRC_FRAMES.mkdir(parents=True, exist_ok=True)
INK_FRAMES.mkdir(parents=True, exist_ok=True)

if not SRC_VIDEO.exists():
    run("yt-dlp", args.url, "-t", "mp4", "-o", SRC_VIDEO.with_suffix(".%(ext)s"))


def probe(*args: str) -> str:
    return subprocess.run(
        [
            *("ffprobe", "-v", "error", "-select_streams", "v:0"),
            *args,
            *("-of", "csv=p=0", SRC_VIDEO),
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


src_framerate = probe("-show_entries", "stream=r_frame_rate")
src_frame_count = int(
    probe("-count_packets", "-show_entries", "stream=nb_read_packets")
)

if len(list(SRC_FRAMES.iterdir())) != src_frame_count:
    run("ffmpeg", "-i", SRC_VIDEO, SRC_FRAMES / "%04d.png")

src_frames = sorted(SRC_FRAMES.iterdir(), key=lambda frame: int(frame.stem))

cv2.namedWindow(WINDOW_TITLE)
cam = cv2.VideoCapture(args.webcam)

try:
    prev_src_frame = None
    prev_src_data = None
    for i, src_frame in enumerate(src_frames):
        print(f"On frame {i + 1:04}/{len(src_frames)}")
        ink_frame = INK_FRAMES / src_frame.name

        data = src_frame.read_bytes()
        if ink_frame.exists():
            print("Output frame already exists, skipping")
        elif prev_src_frame is not None and data == prev_src_data:
            print("Frame unchanged, copying previous capture")
            prev_ink_frame = INK_FRAMES / prev_src_frame.name
            shutil.copy(prev_ink_frame, ink_frame)
        else:
            run(DIR / "show_frame", args.address, src_frame)
            start = time.time()

            while time.time() < start + DISPLAY_DELAY:
                ret, capture = cam.read()
                cv2.imshow(WINDOW_TITLE, capture)
                cv2.waitKey(20)

            cv2.imwrite(ink_frame, capture)

        prev_src_frame = src_frame
        prev_src_data = data
except KeyboardInterrupt:
    print("Aborted, building video from captured frames")

cam.release()
cv2.destroyWindow(WINDOW_TITLE)

run(
    "ffmpeg",
    *("-framerate", src_framerate, "-i", INK_FRAMES / "%04d.png"),
    *("-i", SRC_VIDEO, "-map", "0:v", "-map", "1:a?", "-c:a", "copy", "-shortest"),
    INK_VIDEO,
)
