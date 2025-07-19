#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#set text(size: 16pt * 16)
#align(center, rotate(90deg, data.text, reflow: true))

#if data.feed {
  lib.feed
}
