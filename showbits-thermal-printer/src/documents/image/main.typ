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

// If the image is an odd number of pixels wide, we need to add an extra row of
// pixels (in this case, on the right) to ensure that the image pixels fall on
// screen pixels.
#context {
  let img = image("image.png")
  let width = measure(img).width
  let additional = 2pt * calc.fract(width.pt() / 2)
  align(center, stack(dir: ltr, img, h(additional)))
}

#if data.caption != none {
  align(center, text(size: 32pt, data.caption))
}

#if data.feed {
  lib.feed
}
