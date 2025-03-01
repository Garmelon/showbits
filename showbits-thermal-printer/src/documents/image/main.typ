#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#let dithered = lib.dither(
  read("image.png", encoding: none),
  bright: data.bright,
  algorithm: data.algo,
)

#if data.seamless {
  set page(margin: 0pt)
  dithered
  if data.feed { lib.feed }
} else {
  dithered
  if data.feed { lib.feed }
}
