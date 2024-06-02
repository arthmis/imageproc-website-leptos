/** @type {import('tailwindcss').Config} */
module.exports = {
  // content: ["./dist/*.html"],
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
  // plugins: [],
  plugins: [require('daisyui')],
}

