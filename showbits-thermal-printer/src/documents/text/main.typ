#import "lib.typ";
#show: it => lib.init(it)

#let data = json("data.json")

#if data.force_wrap {
  show regex("."): it => it + sym.zws
  data.text
} else { data.text }

#if data.feed {
  lib.feed
}
