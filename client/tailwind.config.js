/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        albertsans: ['"Albert Sans"', "sans-serif"]
      },
      colors: {
        'hiddn': {
          '50': '#fefaec',
          '100': '#fcf0c9',
          '200': '#f8de8f',
          '300': '#f5c754',
          '400': '#f3b536',
          '500': '#ec9114',
          '600': '#d16c0e',
          '700': '#ad4c10',
          '800': '#8d3b13',
          '900': '#743113',
          '950': '#421806',
        },
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}

