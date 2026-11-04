#import "../prelude.typ": *
#show: set-page
#show: use-unifont

#import "@preview/bulb:0.4.0": dither

#set page(fill: black, margin: (x: 8pt, top: 8pt))
#set text(fill: white)
#show heading: set block(above: 8pt)

// Keep everything to one page
#show: block.with(height: 100%)

#let events = json("events.json")
#for event in events [
  #box(
    fill: white,
    inset: 4pt,
    radius: 4pt,
    text(fill: black, event.date),
  )
  (#event.date_precision),
  #box(
    fill: red,
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
    line(stroke: 1pt + white, length: 100%),
  )
]
