import js from "@eslint/js";
import { defineConfig } from "eslint/config";
import globals from "globals";
import tseslint from "typescript-eslint";

export default defineConfig([
  {
    files: ["**/*.{js,mjs,cjs,ts,mts,cts}"],
    plugins: { js },
    extends: ["js/recommended"],
    languageOptions: {
      globals: globals.browser,
    },
    rules: {
      indent: ["error", 2],
      "max-len": ["error", { code: 100 }],
      semi: ["error", "always"],
    },
  },
  tseslint.configs.recommended,
]);
