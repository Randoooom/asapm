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
  <v-container class='justify-center align-center d-flex'>
    <v-card class='pt-5 pb-5' max-width='400px' width='100%'>
      <v-card-title class='d-block text-center'>
        Login
      </v-card-title>

      <v-card-text>
        <v-alert v-if='error' type='error'>
          Believe in yourself, maybe you can do it!
        </v-alert>

        <v-text-field v-model='loginData.username' label='Username' filled />
        <v-text-field v-model='loginData.password' :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                      filled :type='show ? "text" : "password"' label='Password'
                      @click:append='show = !show' />

        <nuxt-link to='/signup'>
          Create new account
        </nuxt-link>
      </v-card-text>

      <v-card-actions>
        <v-btn text color='primary' :loading='processing' @click='login'>
          Login
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'

@Component({
  name: 'Index',
})
export default class IndexComponent extends Vue {
  show: boolean = false
  error: boolean = false
  processing: boolean = false

  loginData = {
    username: '',
    password: ''
  }

  async login() {
    this.processing = true

    await invoke('login', { data: this.loginData })
    .then(() => {
      this.$router.push('/user/dashboard')
      this.$store.commit('auth/login')
      this.processing = false
    })
    .catch(() => {
      this.error = true
      this.processing = false
    })
  }
}
</script>
