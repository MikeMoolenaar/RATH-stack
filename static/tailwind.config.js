/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../templates/*.{html,js}", "../src/*.rs"],
  plugins: [
    require('tailwindcss'),
    require('autoprefixer'),
    require("daisyui")
  ],
  daisyui: {
    themes: ["light", "night"],
    darkTheme: "night"
  }
}