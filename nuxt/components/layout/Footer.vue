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
  <v-footer color='primary' padless class='pt-5 pb-5'>
    <v-container>
      <v-row
        justify='center'
        no-gutters
      >
        <v-col
          id='theme'
          class='text-center white--text'
          cols='12'
        >
          Copyright {{ new Date().getFullYear() }} — <strong>Randoooom</strong>
          <v-switch v-model='$vuetify.theme.dark' label='Dark' color='accent'/>
        </v-col>
      </v-row>
    </v-container>
  </v-footer>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'

@Component({
  name: 'Footer'
})
export default class FooterComponent extends Vue {
}
</script>

<style lang='sass' scoped>
.v-footer
  position: absolute
  bottom: 0
  left: 0
  right: 0

#theme
  .v-input--switch
    position: absolute
    right: 30px
    width: fit-content
    bottom: 0
</style>
