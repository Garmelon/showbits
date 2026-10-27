#let width = 880pt
#let height = 528pt

#let white = rgb(255, 255, 255)
#let black = rgb(0, 0, 0)
#let red = rgb(200, 0, 0)

#let init(it) = {
  set page(width: width, height: height, margin: (x: 0pt, y: 4pt))
  it
}

#let unifont = ("Unifont", "Unifont-JP", "Unifont Upper")
#let unifont-size = 16pt
#let use-unifont(it) = {
  set text(font: unifont, size: unifont-size, fallback: false)
  set par(
    leading: 6pt, // Between lines
    spacing: 22pt, // Between paragraphs
  )
  show heading: set text(size: 2 * unifont-size)
  show heading: set block(above: 8pt, below: 8pt)
  it
}
