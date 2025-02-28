#import "lib.typ";
#show: it => lib.init(it)

#let data = json("data.json")

#if data.at("force_wrap", default: false) {
  show regex("."): it => it + sym.zws
  data.text
} else { data.text }

#if data.at("feed", default: false) {
  lib.feed
}
