module.exports = {
  purge: [
    "../src/**/*.rs"
  ],
  darkMode: "class",
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [
    require('@tailwindcss/forms'),
  ],
}