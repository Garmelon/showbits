#let width = 880pt
#let height = 528pt

#let unifont = ("Unifont", "Unifont-JP", "Unifont Upper")
#let unifont-size = 16pt

#let init(it) = {
  set page(width: width, height: height, margin: (x: 0pt, y: 4pt))
  it
}
