import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import tailwindcss from '@farmfe/plugin-tailwindcss';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    tailwindcss({
      content: ["./src/**/*.{js,jsx,ts,tsx}"],
      theme: {
        colors: {
          'primary': '#0070f3',
          'secondary': '#0070f3',
          'tertiary': '#0070f3',
          'quaternary': '#0070f3',
          'quinary': '#0070f3',
          'senary': '#0070f3',
        }
      }
    })
  ],
});
