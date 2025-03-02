#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#set par(spacing: 6pt)

#image("image.png")

#if data.rule != none [
  Cellular Automaton
  #h(1fr)
  Rule #data.rule
]

#if data.feed {
  lib.feed
}
