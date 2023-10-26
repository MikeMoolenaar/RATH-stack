/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./*.{html,js}", "../*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "night"],
    darkTheme: "night"
  }
}

