/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ["class", '[data-mode="dark"]'],
  content: ["index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        background: "#1e1e1e",
        primaryText: "#fff",
        secondaryTextColor: "#a0a0a0",
        accent: "#ff4d4d",
        accentSecond: "#53c2da",
        highlight: "#ffd700",
        borders: "#333",
      },
    },
  },
  plugins: [],
};
