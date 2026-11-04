#import "../prelude.typ": *
#show: set-page

#import "@preview/typed-scores:0.6.1": *

#set align(center + horizon)

#score(
  scale: 2.7,
  beams: true,
  bars: (
    (notes: "a4:e b4 - c5 d5 b4:q g4:e a4 ~"),
    (notes: "a4:w", barline: (right: "final")),
    // Shifted version; I feel like the emphasis is wrong.
    // (partial: "1/2", notes: "a4:e b4 - c5 d5 "),
    // (notes: "b4:q g4:e a4 ~ a4:h", barline: (right: "final")),
  ),
)
