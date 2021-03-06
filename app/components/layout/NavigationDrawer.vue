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
  <v-navigation-drawer v-model='drawer' :mini-variant='mini' permanent fixed>
    <v-list-item>
      <v-list-item-icon>
        <v-icon v-if='mini' @click='expand'>
          dashboard
        </v-icon>

        <v-icon v-else @click='mini = true'>
          close
        </v-icon>
      </v-list-item-icon>
    </v-list-item>

    <v-divider />

    <v-list v-if='loggedIn'>
      <v-list-item @click='$router.push("/user/dashboard")'>
        <v-list-item-icon>
          <v-icon>
            list
          </v-icon>
        </v-list-item-icon>

        <v-list-item-title>
          Passwords
        </v-list-item-title>
      </v-list-item>

      <v-list-item @click='$router.push("/user/analytics")'>
        <v-list-item-icon>
          <v-icon>
            analytics
          </v-icon>
        </v-list-item-icon>

        <v-list-item-title>
          Analytics
        </v-list-item-title>
      </v-list-item>

      <v-list-item @click='openGenerator'>
        <v-list-item-icon>
          <v-icon>
            precision_manufacturing
          </v-icon>
        </v-list-item-icon>

        <v-list-item-title>
          Generator
        </v-list-item-title>
      </v-list-item>

      <v-list-item @click='changeTheme'>
        <v-list-item-icon>
          <v-icon>
            {{ dark ? 'light_mode' : 'dark_mode' }}
          </v-icon>
        </v-list-item-icon>

        <v-list-item-title>
          Switch theme
        </v-list-item-title>
      </v-list-item>

      <v-divider />

      <v-list-item @click='logout'>
        <v-list-item-icon>
          <v-icon>
            logout
          </v-icon>
        </v-list-item-icon>

        <v-list-item-title>
          Logout
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'

@Component({
  name: 'NavigationDrawer'
})

export default class NavigationDrawerComponent extends Vue {
  drawer: boolean = true
  mini: boolean = true

  get loggedIn() {
    return this.$store.state.auth.loggedIn
  }

  expand() {
    // only if logged in
    if (!this.loggedIn) return this.$router.push('/')

    this.mini = false
  }

  openGenerator() {
    this.$store.commit('generator/trigger')
  }

  async logout() {
    await invoke('logout')
    .then(() => {
      this.$router.push('/')
      this.$store.commit('auth/logout')
      this.$store.commit('snackbar/emitSnackbar', {
        color: 'success',
        text: 'Logout completed',
        outlined: true
      })
    })
  }

  get dark() {
    // @ts-ignore
    return this.$vuetify.theme.dark
  }

  changeTheme() {
    // @ts-ignore
    this.$vuetify.theme.dark = !this.dark
    this.$store.commit('snackbar/emitSnackbar', {
      color: 'success',
      text: 'Changed theme',
      outlined: true
    })
  }
}
</script>

<style lang='sass'>
.v-navigation-drawer__content
  display: flex
  justify-content: center
  flex-grow: 0
  flex-direction: column

  .v-list-item
    flex: none !important
</style>
