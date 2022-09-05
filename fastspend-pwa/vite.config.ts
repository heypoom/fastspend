import {defineConfig} from 'vite'
import {VitePWA as pwa} from 'vite-plugin-pwa'
import {svelte} from '@sveltejs/vite-plugin-svelte'

import postcss from './postcss.config.cjs'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), pwa({registerType: 'autoUpdate'})],

  css: {postcss},
})
