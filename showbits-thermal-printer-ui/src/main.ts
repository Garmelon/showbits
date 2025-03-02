import { createApp } from "vue";
import App from "./App.vue";

// The type of App contains any in its type parameters, according to vscode.
// Presumably, this is what triggers the lint.
//
// @vue/eslint-config-typescript turns this option off entirely.
// https://github.com/vuejs/eslint-config-typescript/blob/bcdeb741521a718d44dfe77aadcf6d0702b1fd21/src/internals.ts#L139
//
// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
createApp(App).mount("#app");
