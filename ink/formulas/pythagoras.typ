#import "../prelude.typ"
#show: prelude.init

#import "@preview/cetz:0.5.3"

#set text(25mm)
#set align(center + horizon)

#stack(
  dir: ltr,
  spacing: 2mm,

  $
    a^2 + b^2 = text(fill: prelude.red, c^2)
  $,

  cetz.canvas(length: 40mm, {
    import cetz.draw: *
    set-style(stroke: 1mm)

    let theta = 45deg

    // The triangle itself
    // line((0deg, 1), (theta, 1), (180deg, 1), close: true)

    {
      let phi = theta / 2
      set-origin((-1, 0))
      rotate(phi)
      scale(2 * calc.cos(phi))
      rect((0, 0), (1, 1), name: "a")
      set-transform(none)
    }

    {
      let phi = -90deg + theta / 2
      set-origin((1, 0))
      rotate(phi)
      scale(2 * calc.cos(phi))
      rect((0, 0), (-1, 1), name: "b", stroke: none, fill: black)
      set-transform(none)
    }

    {
      set-origin((theta, 1))
      rotate(theta / 2)
      scale(0.2)
      line((-1, 0), (-1, -1), (0, -1))
      set-transform(none)
    }

    rect((-1, 0), (1, -2), name: "c", stroke: none, fill: prelude.red)

    content((name: "a"), $a$)
    content((name: "b"), text(fill: white, $b$))
    content((name: "c"), text(fill: white, $c$))
  }),
)
