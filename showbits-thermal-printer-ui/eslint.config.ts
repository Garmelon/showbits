import eslint from "@eslint/js";
import prettier from "eslint-config-prettier";
import vue from "eslint-plugin-vue";
import { defineConfig } from "eslint/config";
import globals from "globals";
import tseslint from "typescript-eslint";

export default defineConfig(
  { files: ["**/*.{js,mjs,cjs,ts,vue}"] },
  { ignores: ["dist/"] },

  { languageOptions: { globals: { ...globals.browser, ...globals.node } } },

  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
        extraFileExtensions: ["vue"],
      },
    },
  },

  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
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
      "@typescript-eslint/naming-convention": [
        "warn",
        // Default settings
        // https://typescript-eslint.io/rules/naming-convention/#options
        {
          selector: "default",
          format: ["camelCase"],
          leadingUnderscore: "allow",
          trailingUnderscore: "allow",
        },
        {
          selector: "import",
          format: ["camelCase", "PascalCase"],
        },
        {
          selector: "variable",
          format: ["camelCase", "UPPER_CASE"],
          leadingUnderscore: "allow",
          trailingUnderscore: "allow",
        },
        {
          selector: "typeLike",
          format: ["PascalCase"],
        },
        // Prevent warnings in some specific cases
        {
          selector: "objectLiteralProperty",
          format: null,
        },
      ],

      // https://eslint.vuejs.org/rules/
      "vue/block-lang": ["error", { script: { lang: "ts" } }],
      "vue/block-order": ["error", { order: ["script", "template", "style"] }],
      "vue/component-api-style": ["error", ["script-setup"]],
      "vue/eqeqeq": "error",
      "vue/v-for-delimiter-style": ["error", "of"],
    },
  },
);
