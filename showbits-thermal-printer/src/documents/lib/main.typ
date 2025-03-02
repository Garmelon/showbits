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

////////////
// Plugin //
////////////

#import plugin("plugin.wasm") as p

#let _number_to_bytes(n) = int(n).to-bytes(size: 8)

#let _bool_to_bytes(b) = _number_to_bytes(if b { 1 } else { 0 })

#let _str_to_bytes(s) = {
  bytes(s)
}

#let _length_to_bytes(l) = {
  let l = l.pt()
  let n = if l > 10000 { -1 } else { int(l) }
  _number_to_bytes(n)
}

#let dither(
  data,
  bright: true,
  algorithm: "floyd-steinberg",
) = layout(size => {
  let dithered = p.dither(
    data,
    _length_to_bytes(size.width),
    _length_to_bytes(size.height),
    _bool_to_bytes(bright),
    _str_to_bytes(algorithm),
  )
  image(dithered)
})
