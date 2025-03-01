#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#image("image.png")

#if data.feed {
  lib.feed
}
