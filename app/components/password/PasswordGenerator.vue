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
  <v-bottom-sheet v-model='open'>
    <v-card>
      <v-card-text>
        <v-container>
          <v-otp-input v-model='password' color='secondary' class='mt-8 mb-16' :length='data.length' readonly @click='copyPassword({ password })' />

          <v-slider v-model='data.length' label='Length' prepend-icon='straighten' thumb-label='always' ticks min='12'
                    max='36' />
          <v-row class='d-flex justify-center'>
            <v-col cols='12' sm='2' md='2'>
              <v-switch v-model='data.letters' label='Letters' />
            </v-col>

            <v-col cols='12' sm='2' md='2'>
              <v-switch v-model='data.numbers' label='Numbers' />
            </v-col>

            <v-col cols='12' sm='2' md='2'>
              <v-switch v-model='data.symbols' label='Symbols' />
            </v-col>

            <v-col cols='12'>
              <v-btn outlined color='primary' @click='save'>
                Save as your default
              </v-btn>

              <v-btn class='ml-5' outlined color='secondary' @click='resetToDefault'>
                Reset to default
              </v-btn>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
    </v-card>
  </v-bottom-sheet>
</template>

<script lang='ts'>
import Component, { mixins } from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'
import { Watch } from 'vue-property-decorator'
import PasswordUtil from '~/mixins/PasswordUtil'

@Component({
  name: 'PasswordGenerator'
})

export default class PasswordGeneratorComponent extends mixins(PasswordUtil) {
  data = { ...this.defaultGenerator }
  password: string = ''

  @Watch('data', { deep: true })
  async onDataChange() {
    await this.generate()
  }

  @Watch('open', { immediate: true })
  async onOpenChange() {
    await this.generate()
  }

   async generate() {
    this.password = await invoke('generate_password', { generator: this.data })
      .then(value => value as string)
  }

  get open() {
    return this.$store.state.generator.open
  }

  set open(_) {
    this.$store.commit('generator/trigger')
  }

  get defaultGenerator() {
    return this.$store.state.generator.default
  }

  resetToDefault() {
    this.data = { ...this.defaultGenerator }

    this.$store.commit('snackbar/emitSnackbar', { color: 'success', outlined: true, text: 'Generator has reset to default' })
  }

  async save() {
    await invoke('update_generator', { generator: this.data })
      .then(async () => {
        // update the entire store
        await this.$store.dispatch('password/fetchData')

        // show snackbar
        this.$store.commit('snackbar/emitSnackbar', {
          color: 'success',
          outlined: true,
          text: 'Saved as your default generator'
        })
      })
  }
}
</script>
