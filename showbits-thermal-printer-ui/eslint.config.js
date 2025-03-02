import eslint from "@eslint/js";
import prettier from "eslint-config-prettier";
import vue from "eslint-plugin-vue";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
  { files: ["**/*.{js,mjs,cjs,ts,vue}"] },
  { ignores: ["dist/"] },

  { languageOptions: { globals: { ...globals.browser, ...globals.node } } },

  eslint.configs.recommended,
  ...tseslint.configs.strict,
  ...tseslint.configs.stylistic,
  ...vue.configs["flat/recommended"],
  prettier,

  {
    files: ["**/*.vue"],
    languageOptions: { parserOptions: { parser: tseslint.parser } },
  },

  {
    rules: {
      // https://eslint.org/docs/latest/rules/
      eqeqeq: "error",

      // https://typescript-eslint.io/rules/
      "@typescript-eslint/explicit-function-return-type": "error",

      // https://eslint.vuejs.org/rules/
      "vue/block-lang": ["error", { script: { lang: "ts" } }],
      "vue/block-order": ["error", { order: ["script", "template", "style"] }],
      "vue/component-api-style": ["error", ["script-setup"]],
      "vue/eqeqeq": "error",
      "vue/v-for-delimiter-style": ["error", "of"],
    },
  },
);
