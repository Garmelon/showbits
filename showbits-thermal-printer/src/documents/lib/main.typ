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
    leading: 6pt, // Between lines
    spacing: 22pt, // Between paragraphs
  )
  show heading: set text(size: 32pt)
  show heading: set block(above: 8pt, below: 8pt)
  it
}

// Determined by experiments so that the top and bottom white border are roughly
// the same size after tearing off the paper.
#let feed = v(64pt + 32pt)

#let chain(..args) = {
  assert(args.pos().len() > 0, message: "args required")
  args.pos().slice(0, -1).rev().fold(args.pos().at(-1), (x, f) => f(x))
}
