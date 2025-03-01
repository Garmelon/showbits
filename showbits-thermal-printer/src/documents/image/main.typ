#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#show: it => if data.seamless {
  set page(margin: 0pt)
  it
} else { it }

#if data.title != none {
  align(center, text(size: 32pt, data.title))
}

#lib.dither(
  read("image.png", encoding: none),
  bright: data.bright,
  algorithm: data.algo,
)

#if data.caption != none {
  align(center, text(size: 32pt, data.caption))
}

#if data.feed {
  lib.feed
}
