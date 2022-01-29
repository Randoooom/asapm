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
        {{ password.name || 'Unnamed password' }}
        <v-icon v-if='!editable' class='ml-2' small color='primary' @click='editMode = true'>edit</v-icon>
      </v-card-title>

      <v-card-text>
        <v-row>
          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.name' :readonly='!editable' type='text' filled label='Name' />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.url' :readonly='!editable' type='text' filled label='URL' />
          </v-col>

          <v-col cols='12'>
            <v-text-field v-model='password.login' :readonly='!editable' type='text' label='Login' filled />
          </v-col>

          <v-col cols='12'>
            <v-textarea v-model='password.description' :readonly='!editable' label='Description' rows='3' filled />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-text-field v-model='password.password' :disabled='!editable' label='Password' filled :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                          :type='show ? "text" : "password"'
                          :rules='[required]'
                          :hint='strength' @input='onPasswordInput' @click:append='show = !show' />
          </v-col>

          <v-col cols='12' sm='6' md='6'>
            <v-btn :disabled='!password.password || password.password.length === 0' height='56px' color='primary' outlined @click='copyPassword(password)'>
              Copy
            </v-btn>

            <v-btn :disabled='!editable' height='56px' class='ml-3' outlined color='primary' @click='generate'>
              Regenrate from generator
            </v-btn>
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions>
        <v-btn :disabled='!editable' text color='primary' @click='savePassword'>
          {{ password.uuid ? 'Save' : 'Create' }}
        </v-btn>

        <v-spacer />

        <v-btn :disabled='!password.uuid' color='red' outlined @click='deletePassword'>
          Delete
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
import PasswordUtil from '~/mixins/PasswordUtil'

@Component({
  name: 'PasswordDialog'
})

export default class PasswordDialogComponent extends mixins(FormValidator, PasswordUtil) {
  show: boolean = false
  editMode: boolean = false
  strength: string = ''

  get editable() {
    // auto allow edit while creating new password
    return this.editMode || !this.password.uuid
  }

  get password() {
    // make deep copy
    return { ...this.$store.state.password.currentPassword }
  }

  set password(data: any) {
    this.$store.commit('password/pushPassword', data)
  }

  @Prop({ type: Boolean, required: true })
  value!: boolean

  @Watch('password', { deep: true })
  async onPasswordInput(password: any) {
    if (!password?.password && typeof password !== 'string') return
    const data = typeof password === 'string' ? password : password.password

    this.strength = await this.getPasswordStrength(data).then(value => value)
  }

  @Watch('value')
  onValueChange(value: boolean) {
    // always hide the password for default
    this.show = false

    // reset on disable
    if (!value) {
      // handle the out animation
      setTimeout(() => {
        // break up on reactivate
        if (this.value) return

        this.password = undefined
        this.editMode = false
      }, 200)

      return
    }

    // init password on undefined
    if (!this.password)
      this.password = {}
  }

  /**
   * generate a new password from the default generator with the length of 32
   */
  async generate() {
    this.password = Object.assign(this.password,
      { password: await this.generatePassword(32).then(value => value).catch(() => 'bad') }
    )
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

  /**
   * start the password deletion dialog
   */
  deletePassword() {
    // the confirmation callback
    const callback = async () => {
      await invoke('delete_password', { data: this.password })
        .then(async () => {
          this.$store.commit('snackbar/emitSnackbar', { color: 'success', text: 'Password deleted', outlined: true })
          this.$emit('input', false)
          await this.$store.dispatch('password/fetchData')
        })
    }

    this.confirmAction({ callback, text: 'Are you sure you want to delete the password? You can not get it back!' })
  }
}
</script>
