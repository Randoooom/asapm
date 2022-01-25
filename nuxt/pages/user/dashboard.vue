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
  <v-container>
    <v-card>
      <v-card-title>
        <v-dialog v-model='dialog' max-width='500px'>
          <v-card>
            <v-card-title>
              Create new password
            </v-card-title>

            <v-card-text>
              <v-row>
                <v-col cols='12' sm='6' md='6'>
                  <v-text-field type='text' label='Login' filled />
                </v-col>

                <v-col cols='12' sm='6' md='6'>
                  <v-text-field type='password' label='password' filled />
                </v-col>
              </v-row>
            </v-card-text>
          </v-card>
        </v-dialog>

        <v-btn color='primary' @click='dialog = true'>
          Create new password
        </v-btn>
      </v-card-title>

      <v-card-text>
        <v-list>
          <password v-for='password in passwords' :key='password.uuid' :password='password' />
        </v-list>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'

@Component({
  name: 'Dashboard',
  components: {
    'password': () => import('~/components/Password.vue')
  }
})
export default class DashboardComponent extends Vue {
  passwords: any[] = []
  dialog: boolean = false

  async mounted() {
    // @ts-ignore
    this.passwords = await invoke('get_passwords')
      .then(value => value)
  }
}
</script>
