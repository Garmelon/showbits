<template>
  <div class="glow">
    <div class="strip">
      <h2>Thing</h2>
      Hello World
      <br />
      World Hello
      <br />
      <input type="file" />
      <input />
      <br />
      <button>Foo</button>
      <button>Foo</button>
      <button>Foo</button>
    </div>
  </div>
</template>

<style scoped>
.strip {
  box-sizing: content-box;
  width: 384px;
  padding: 32px;

  background-color: white;
  mask: conic-gradient(from 100deg at top, #000 160deg, #0000 160deg)
    center/64px;
}

.glow {
  filter: drop-shadow(0 0 50px rgb(255, 238, 214));
}
</style>

<style>
@font-face {
  font-family: "Unifont";
  src: url("/fonts/unifont-15.1.05.otf") format("opentype");
}
@font-face {
  font-family: "Unifont-JP";
  src: url("/fonts/unifont_jp-15.1.05.otf") format("opentype");
}
@font-face {
  font-family: "Unifont Upper";
  src: url("/fonts/unifont_upper-15.1.05.otf") format("opentype");
}

* {
  font-family: "Unifont", "Unifont-JP", "Unifont Upper", monospace;
  font-size: 16px;
  margin: 0;
  padding: 0;
}

h2 {
  font-size: 32px;
}

body {
  margin: 0;
  min-height: 100dvh;

  display: flex;
  flex-direction: column-reverse;
  align-items: center;

  background-color: black;
}
</style>
