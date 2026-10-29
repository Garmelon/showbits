#import "@preview/bulb:0.4.0": dither

#import "../prelude.typ"
#show: prelude.init
#show: prelude.use-unifont

#set page(fill: prelude.black, margin: (x: 8pt, top: 8pt))
#set text(fill: prelude.white)
#show heading: set block(above: 8pt)

// Keep everything to one page
#show: block.with(height: 100%)

#let events = json("events.json")
#for event in events [
  #box(
    fill: prelude.white,
    inset: 4pt,
    radius: 4pt,
    text(fill: prelude.black, event.date),
  )
  (#event.date_precision),
  #box(
    fill: prelude.red,
    inset: 4pt,
    radius: 4pt,
    event.location,
  )

  = #event.name

  #for part in event.description.split(regex("\r?\n\r?\n")) { par(part) }

  // Block above/below overrides par spacing
  #block(
    above: 8.5pt,
    below: 8.5pt,
    line(stroke: 1pt + prelude.white, length: 100%),
  )
]
