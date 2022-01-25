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
        Account creation
      </v-card-title>

      <v-card-subtitle class='mt-2'>
        Please insert the requested information.
      </v-card-subtitle>

      <v-card-text>
        <v-text-field v-model='data.username' filled label='Username' :rules='[required]' />
        <v-text-field v-model='data.password' :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                      filled :type='show ? "text" : "password"' label='Password'
                      :rules='[required]' @click:append='show = !show' />
        <v-text-field v-model='confirm' :rules='[matchPassword, required]'
                      :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                      filled :type='show ? "text" : "password"' label='Confirm password'
                      @click:append='show = !show' />
      </v-card-text>

      <v-card-actions>
        <v-btn text color='primary' @click='signup'>
          Execute
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import Component, { mixins } from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'
import FormValidator from '~/mixins/FormValidator'

@Component({
  name: 'Signup',
  auth: 'guest'
})
export default class SignupComponent extends mixins(FormValidator) {
  show: boolean = false
  confirm: string = ''

  get matchPassword() {
    return (confirm: string) => confirm === this.data.password || 'Does not match!'
  }

  data = {
    username: '',
    password: ''
  }

  async signup() {
    return await invoke('signup', { data: this.data })
  }
}
</script>
