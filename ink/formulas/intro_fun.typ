#import "../prelude.typ": *
#show: set-page

#import "@preview/curryst:0.6.0": prooftree, rule, rule-set

#set align(center + horizon)
#set text(size: 3cm)

#prooftree(rule(
  name: [Abs],
  $Gamma, text(red, x : alpha) tack.r t : beta$,
  $Gamma tack.r lambda text(red, x). t
  : text(red, alpha) -> beta$,
))
