#let width = 384pt

#let init(it) = {
  set page(
    width: width,
    height: auto,
    margin: (x: 0pt, y: 4pt),
  )
  set text(
    font: ("Unifont", "Unifont-JP", "Unifont Upper"),
    size: 16pt,
    fallback: false,
  )
  set par(
    leading: 8pt, // Between lines
    spacing: 26pt, // Between paragraphs
  )
  show heading: set text(size: 32pt)
  show heading: set block(above: 8pt, below: 8pt)
  it
}

// Determined by experiments so that the top and bottom white border are roughly
// the same size after tearing off the paper.
#let feed = v(64pt + 32pt)
