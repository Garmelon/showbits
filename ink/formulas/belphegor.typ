#import "../prelude.typ"
#show: prelude.init

#set align(center + horizon)
#set text(size: 3cm)

// https://en.wikipedia.org/wiki/Belphegor%27s_prime

// $
//   #hide($0$) 1000000000 \
//   0000 text(prelude.red, 666) 0000 \
//   0000000001 #hide($0$) \
// $

// $
//   & 10000000000 \
//   & 000 text(prelude.red, 666) 00000 \
//   & 000000001 in PP \
// $

#set math.mat(delim: none)
$
  mat(
    #hide($0$) 1000000000;
    0000 text(prelude.red, 666) 0000;
    0000000001 #hide($0$);
  ) quad in PP
$
