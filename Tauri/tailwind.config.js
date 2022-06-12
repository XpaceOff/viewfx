module.exports = {
    //The content section is where you configure the paths to all 
    // files that contain Tailwind class names.
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
    },
  },
  plugins: [
    require('tailwind-scrollbar'),
  ],
}
