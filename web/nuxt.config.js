import tailwindForm from '@tailwindcss/forms';

export default {
  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: 'CryptoMarket - Cryptocurrency Market',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
      { name: 'format-detection', content: 'telephone=no' },
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }],
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
      '~static/global.css',
  ],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/eslint
    // '@nuxtjs/eslint-module',
    // https://go.nuxtjs.dev/tailwindcss
    '@nuxtjs/tailwindcss',
  ],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    // https://go.nuxtjs.dev/axios
    '@nuxtjs/axios',
    // https://go.nuxtjs.dev/pwa
    '@nuxtjs/pwa',
  ],

  server: {
    host: '0.0.0.0',
  },

  // axios, api is at 127.0.0.1:8000 so we need to proxy it
  axios: {
    proxy: true,
  },

  proxy: {
    // Simple proxy
    "/api/": {
      target: "http://127.0.0.1:8000/",
      pathRewrite: { "^/api/": "/" },
    },
  },

  // PWA module configuration: https://go.nuxtjs.dev/pwa
  pwa: {
    manifest: {
      lang: 'fr',
    },
    icon: {
      source: 'static/favicon.ico',
    },
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {},

  // Config for tailwindcss
  tailwindcss: {
    config: {
      content: [],
      plugins: [tailwindForm]
    }
  }
}
