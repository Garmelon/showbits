#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

= Catfishing (day #data.day)

#for article in data.articles [
  #v(24pt)
  #for cat in article.categories [
    - #cat
  ]
  #v(24pt)
  #line(length: 100%)
]

#if data.feed {
  lib.feed
}
