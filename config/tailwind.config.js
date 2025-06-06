/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        './public/*.html',
        './app/helpers/**/*.rb',
        './app/javascript/**/*.js',
        './app/views/**/*',
    ],
    theme: {
        extend: {}
    },
    plugins: [require('flowbite/plugin')],
}