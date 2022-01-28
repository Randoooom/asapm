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
  <v-dialog v-if='password' v-model='value' max-width='800px' @input='$emit("input", $event.target)'>
    <v-card>
      <v-card-title>
        {{ password.uuid ? 'Edit' : 'Create new' }} password
      </v-card-title>

      <v-card-text>
        <v-row>
          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.name' type='text' filled label='Name' />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.url' type='text' filled label='URL' />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.login' type='text' label='Login' filled />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.password' label='Password' filled
                          :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                          :type='show ? "text" : "password"'
                          :rules='[required]' @click:append='show = !show' />
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions>
        <v-btn text color='primary' @click='savePassword'>
          {{ password.uuid ? 'Save' : 'Create' }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang='ts'>
import Component, { mixins } from 'vue-class-component'
import { Prop, Watch } from 'vue-property-decorator'
import { invoke } from '@tauri-apps/api/tauri'
import FormValidator from '~/mixins/FormValidator'

@Component({
  name: 'PasswordDialog'
})

export default class PasswordDialogComponent extends mixins(FormValidator) {
  show: boolean = false

  get password() {
    // make deep copy
    return { ...this.$store.state.password.currentPassword }
  }

  set password(data: any) {
    this.$store.commit('password/pushPassword', data)
  }

  @Prop({ type: Boolean, required: true })
  value!: boolean

  @Watch('value')
  onValueChange(value: boolean) {
    // just clear current password
    if (!value) return (this.password = undefined)

    // init password on undefined
    if (!this.password)
      this.password = {}
  }

  /**
   * save the password on the local disk
   */
  async savePassword() {
    // handle new password
    if (!this.password.uuid) {
      // create new
      const newData = await invoke('new_password')
        .then(value => value)

      // merge data
      this.password = Object.assign(newData, this.password)
    }

    // save the password
    await invoke('update_password', { data: this.password })
      .then(async () => {
        this.$emit('input', false)
        await this.$store.dispatch('password/fetchData')
      })
  }
}
</script>
