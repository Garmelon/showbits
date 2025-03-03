#import "@preview/oxifmt:0.2.1": strfmt
#import "@preview/suiji:0.3.0": *
#import "lib/main.typ" as lib;
#show: it => lib.init(it)

#let data = json("data.json")
#let rng = gen-rng(data.seed)

#let file_series(n, fmt) = array.range(n).map(n => strfmt(fmt, n))

#let good_covers = file_series(data.covers, "eggs/good/cover_{:02}.png")
#let good_patterns = file_series(data.patterns, "eggs/good/pattern_{:02}.png")
#let bad_covers = file_series(data.bad_covers, "eggs/bad/cover_{:02}.png")
#let bad_patterns = file_series(data.bad_patterns, "eggs/bad/pattern_{:02}.png")

// Always generate random value to so that egg looks the same whether we chose
// the mode directly or randomly.
#let (rng, val) = random(rng)

#let (covers, patterns) = if data.mode == "good" {
  (good_covers, good_patterns)
} else if data.mode == "bad" {
  (bad_covers, bad_patterns)
} else if val < 1 / 8 {
  (bad_covers, bad_patterns)
} else {
  (good_covers, good_patterns)
}

#context {
  let (rng, cover) = choice(rng, covers)
  let cover = image(cover, width: lib.width)
  let cover_size = measure(cover)

  let pattern_stack_height = 0pt
  let pattern_stack = while pattern_stack_height < cover_size.height {
    let pattern = ()
    (rng, pattern) = choice(rng, patterns)
    let pattern = image(pattern, width: cover_size.width)
    pattern_stack_height += measure(pattern).height
    (pattern,)
  }

  box(
    height: cover_size.height,
    clip: true,
    stack(dir: ttb, ..pattern_stack),
  )

  place(top + left, cover)
}

#if data.feed {
  lib.feed
}
