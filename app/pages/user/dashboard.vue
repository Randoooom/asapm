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
        <password-dialog v-model='dialog' />

        <v-btn color='primary' outlined @click='dialog = true'>
          <v-icon>
            add
          </v-icon>
          Create new password
        </v-btn>
      </v-card-title>

      <v-card-text>
        <v-list flat>
          <v-list-item-group>
            <template v-for='password in passwords'>
              <v-list-item :key='password.uuid'>
                <v-list-item-avatar tile color='transparent'>
                  <v-icon small @click='editPassword(password)'>
                    edit
                  </v-icon>
                </v-list-item-avatar>

                <v-list-item-title>
                  {{ password.name }}
                </v-list-item-title>

                <v-list-item-content>
                  {{ password.login }}
                </v-list-item-content>

                <v-list-item-icon>
                  <a v-if='password.url && password.url.length !== 0' :href='password.url' target='_blank'
                     @click='confirmOpen'>
                    <v-icon color='white' small>
                      link
                    </v-icon>
                  </a>
                </v-list-item-icon>

                <v-list-item-icon>
                  <v-icon small @click.prevent='copyPassword(password)'>
                    content_copy
                  </v-icon>
                </v-list-item-icon>
              </v-list-item>
              <v-divider :key='password.uuid' />
            </template>
          </v-list-item-group>
        </v-list>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import Component, { mixins } from 'vue-class-component'
import PasswordUtil from '~/mixins/PasswordUtil'

@Component({
  name: 'Dashboard',
  components: {
    'password-dialog': () => import('~/components/password/PasswordDialog.vue')
  }
})
export default class DashboardComponent extends mixins(PasswordUtil) {
  dialog: boolean = false

  async mounted() {
    await this.$store.dispatch('password/fetchData')
  }

  editPassword(password: any) {
    // set current password
    this.$store.commit('password/pushPassword', password)
    // activate dialog
    this.dialog = true
  }

  get passwords() {
    return this.$store.state.password.passwords
  }

  confirmOpen() {
    this.$store.commit('snackbar/emitSnackbar', {
      color: 'success',
      outlined: true,
      text: 'Link opened in default browser'
    })
  }
}
</script>
