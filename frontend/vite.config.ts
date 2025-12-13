import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import { viteSingleFile } from "vite-plugin-singlefile";

// Vite + Tailwind 4 + DaisyUI 5 + single-file build
export default defineConfig({
  plugins: [
    tailwindcss(),
    viteSingleFile()
  ]
});
