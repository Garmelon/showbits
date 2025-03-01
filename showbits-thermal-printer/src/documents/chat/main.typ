#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")

#let max_width = 32 * 8pt + 4pt
#let limit_width(body) = context {
  let width = measure(body).width
  if width > max_width { box(body, width: max_width) } else { body }
}

// This way, the top line of the username box looks better.
#v(4pt)

#par(hanging-indent: 32pt)[
  #limit_width(
    box(
      height: 10pt,
      clip: true,
      stroke: 1pt + black,
      inset: (x: 2pt),
      outset: (y: 3pt + .5pt, x: -.5pt),
      {
        // Ensure all characters that fit on the line are displayed on the line.
        // We don't want a half-empty box.
        show regex("."): it => it + sym.zws
        data.username
      },
    ),
  )
  #h(-4pt)
  #data.content
]

#if data.feed {
  lib.feed
}
