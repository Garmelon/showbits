#import "@preview/oxifmt:0.2.1": strfmt
#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")
#let date = datetime(year: data.year, month: data.month, day: 1)

#let month_length = 32 - (date + duration(days: 31)).day()

#let head(name) = text(size: 32pt, name)
#let empty = box()
#let day(n) = box(
  width: 100%,
  height: 100%,
  stroke: 2pt + black,
  strfmt("{:02}", n),
)

#align(center + horizon)[
  #set par(spacing: 8pt)

  Ankreuzkalender #strfmt("{:04}-{:02}", date.year(), date.month())

  #grid(
    columns: (50pt,) * 7,
    rows: 50pt,
    gutter: 4pt,
    head[Mo], head[Di], head[Mi], head[Do], head[Fr], head[Sa], head[So],
    ..for _ in range(date.weekday() - 1) { (empty,) },
    ..for i in range(month_length) { (day(i + 1),) },
  )
]

#if data.feed {
  lib.feed
}
