#import "@preview/bulb:0.4.0": dither

#import "../prelude.typ"
#show: prelude.init
#show: prelude.use-unifont

#set page(fill: prelude.black, margin: 8pt)
#set text(fill: prelude.white)
#set block(above: 8pt, below: 0pt)

// Keep everything to one page
#show: block.with(height: 100%)

#let events = json("events.json")
#for event in events {
  let img = image(dither(
    path(event.image),
    colors: (prelude.white, prelude.black, prelude.red),
    method: "floyd-steinberg",
    size: 8 * int(prelude.unifont-size.pt()),
  ))

  grid(
    columns: (auto, 1fr),
    gutter: 8pt,
    img,
    [
      #box(fill: prelude.red, inset: 2pt, text(fill: prelude.white, event.date)),
      #event.location

      = #event.name
      #v(8pt)

      #event.description
    ],
  )

  v(0.5pt)
  line(stroke: 1pt + prelude.white, length: 100%)
  v(0.5pt)
}
