/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      title: ["Raleway", "sans-serif"],
      line: ["Bahnschrift", "sans-serif"],
      paragraph: ["Roboto", "sans-serif"],
      serif: ["Bree\\ Serif", "serif"],
      mono: ["Fira\\ Code", "monospace"],
    },
    colors: {
      purple: "#6128FF",
      beige: "#FFF3E0",
      yellow: "#FFF157",
      darkpurple: "#290029",
      darkgray: "#202220",
    },
    extend: {
      dropShadow: {
        header: [
          '0 8px 6px rgb(0 0 0 / 0.1)',
          '0 4px 3px rgb(0 0 0 / 0.4)',
        ],
        footer: [
          '0 -8px 6px rgb(0 0 0 / 0.1)',
          '0 -4px 3px rgb(0 0 0 / 0.4)',
        ],
      },
    },
  },
  plugins: [],
}
