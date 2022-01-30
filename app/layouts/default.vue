<!--
  - MIT LICENSE
  -
  - Copyright (c) 2022 Randoooom
  -
  -
  - Permission is hereby granted, free of charge, to any person obtaining a copy
  - of this software and associated documentation files (the "Software"), to deal
  - in the Software without restriction, including without limitation the rights
  - to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  - copies of the Software, and to permit persons to whom the Software is
  - furnished to do so, subject to the following conditions:
  -
  - The above copyright notice and this permission notice shall be included in all
  - copies or substantial portions of the Software.
  -
  - THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  - IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  - FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  - AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  - LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  - OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  - SOFTWARE.
  -->

<template>
  <v-app>
    <navigation-drawer />
    <nuxt class='mt-8' />
    <password-generator />

    <v-snackbar v-model='snackbar' :color='options.color' :outlined='options.outlined' timeout='1000'>
      {{ options.text }}
    </v-snackbar>

    <confirmation-dialog />
  </v-app>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'

@Component({
  name: 'Default',
  components: {
    'navigation-drawer': () => import('~/components/layout/NavigationDrawer.vue'),
    'password-generator': () => import('~/components/password/PasswordGenerator.vue')
  }
})
export default class DefaultLayout extends Vue {
  get snackbar() {
    return this.$store.state.snackbar.active
  }

  get options() {
    return this.$store.state.snackbar
  }

  set snackbar(active: boolean) {
    this.$store.commit('snackbar/setActive', active)
  }
}
</script>

<style lang='sass'>
html
  .v-application
    background-image: url("~/assets/MountainWallpaper.jpg") !important
    background-repeat: no-repeat !important
    background-position: center !important
    background-size: cover !important
    background-attachment: fixed !important

    .v-application--wrap
      opacity: 0.8

      *
        .v-card
          opacity: 0.96

  .theme--dark.v-application
    .v-application--wrap
      background: black

  a
    text-decoration: none
</style>
