#import "@preview/bulb:0.4.0": dither

#import "../prelude.typ"
#show: prelude.init
#show: prelude.use-unifont

#let info = json("info.json")

#align(center + top)[
  = #text(fill: prelude.red)[#info.num] #info.title
]

#{
  set align(center + horizon)
  show: block.with(height: 1fr)
  layout(size => {
    let max_w = calc.floor(size.width.pt())
    let max_h = calc.floor(size.height.pt())
    let w_by_max_h = calc.floor(max_h / info.height * info.width)
    let h_by_max_w = calc.floor(max_w / info.width * info.height)
    // Includes original image size so the image isn't stretched if it's small.
    let size = calc.min(
      calc.max(info.width, info.height),
      calc.max(max_w, h_by_max_w),
      calc.max(w_by_max_h, max_h),
    )
    let dithered = image(dither(
      path("image.png"),
      colors: (prelude.white, prelude.black, prelude.red),
      method: "floyd-steinberg",
      size: size,
    ))
    // The dithered image must be aligned to the pt grid.
    let dithered_size = measure(dithered)
    let dw = calc.rem(dithered_size.width.pt(), 2) * 1pt
    let dh = calc.rem(dithered_size.height.pt(), 2) * 1pt
    stack(dir: ttb, stack(dir: ltr, dithered, h(dw)), v(dh))
  })
}

#align(center + bottom)[#info.alt]
