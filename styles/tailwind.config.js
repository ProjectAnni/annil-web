module.exports = {
  purge: ["./src/**/*.rs", "./index.html", "./index_dev.html"],
  darkMode: "class",
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [require("@tailwindcss/forms")],
};
