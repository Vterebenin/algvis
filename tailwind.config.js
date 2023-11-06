/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "./src/**/*.rs"
  ],
  theme: {
    colors: {
      midnight: '#3c405b',
      pumpkin: '#df7a5e',
      breeze: '#f4f1de',
      deepforest: '#82b29a',
    },
    extend: {},
  },
  plugins: [],
}
