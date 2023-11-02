/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../templates/*.{html,js}", "../src/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "night"],
    darkTheme: "night"
  }
}

