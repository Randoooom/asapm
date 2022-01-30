import colors from 'vuetify/es5/util/colors'

export default {
  // Target: https://go.nuxtjs.dev/config-target
  target: 'static',

  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: 'ASAPM',
    htmlAttrs: {
      lang: 'en'
    },
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
      { name: 'format-detection', content: 'telephone=no' }
    ],
    link: [{ rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }]
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: ['material-design-icons-iconfont/dist/material-design-icons.css'],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: ['~/plugins/clipboard.js'],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/typescript
    '@nuxt/typescript-build',
    // https://go.nuxtjs.dev/stylelint
    '@nuxtjs/stylelint-module',
    // https://go.nuxtjs.dev/vuetify
    '@nuxtjs/vuetify'
  ],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [],

  // Vuetify module configuration: https://go.nuxtjs.dev/config-vuetify
  vuetify: {
    treeShake: true,
    customVariables: ['~/assets/variables.sass'],
    icons: { iconFont: 'md' },
    theme: {
      dark: true,
      themes: {
        light: {
          primary: colors.teal,
          secondary: colors.cyan,
          accent: colors.deepPurple,
          error: colors.pink,
          warning: colors.amber,
          info: colors.indigo,
          success: colors.green
        },
        dark: {
          primary: colors.teal.darken3,
          secondary: colors.cyan.darken3,
          accent: colors.deepPurple.darken3,
          error: colors.pink.darken3,
          warning: colors.amber.darken3,
          info: colors.indigo.darken3,
          success: colors.green.darken3
        }
      }
    }
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
    friendlyErrors: false,
    transpile: ['@tauri-apps/api/tauri']
  },

  generate: {
    fallback: true
  },

  loading: false
}
