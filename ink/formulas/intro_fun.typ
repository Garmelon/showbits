#import "../prelude.typ"
#show: prelude.init

#import "@preview/curryst:0.6.0": prooftree, rule, rule-set

#set align(center + horizon)
#set text(size: 3cm)

#prooftree(rule(
  name: [Abs],
  $Gamma, text(prelude.red, x : alpha) tack.r t : beta$,
  $Gamma tack.r lambda text(prelude.red, x). t
  : text(prelude.red, alpha) -> beta$,
))
