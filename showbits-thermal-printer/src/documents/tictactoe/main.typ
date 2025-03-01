#import "@preview/cetz:0.3.2"
#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#cetz.canvas(
  length: lib.width,
  {
    import cetz.draw: *
    line((1 / 3, 0), (1 / 3, 1), stroke: 4pt)
    line((2 / 3, 0), (2 / 3, 1), stroke: 4pt)
    line((0, 1 / 3), (1, 1 / 3), stroke: 4pt)
    line((0, 2 / 3), (1, 2 / 3), stroke: 4pt)
  },
)

#if data.feed {
  lib.feed
}
