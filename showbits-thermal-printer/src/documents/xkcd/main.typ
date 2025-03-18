#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#align(center)[
  xkcd #data.number
  #v(-12pt)
  #text(size: 32pt, data.title)
]

#if data.dither {
  // If the image is an odd number of pixels wide, we need to add an extra row
  // of pixels (in this case, on the right) to ensure that the image pixels fall
  // on screen pixels.
  context {
    let img = image("image.png")
    let width = measure(img).width
    let additional = 2pt * calc.fract(width.pt() / 2)
    align(center, stack(dir: ltr, img, h(additional)))
  }
} else {
  image("image.png", width: lib.width)
}

#align(center, data.alt)

#if data.feed {
  lib.feed
}
