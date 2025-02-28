#let init(it) = {
  set page(
    width: 384pt,
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
  it
}

// Determined by experiments so that the top and bottom white border are roughly
// the same size after tearing off the paper.
#let feed = v(64pt + 32pt)

#import plugin("plugin.wasm") as plugin
