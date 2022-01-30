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
  <v-list flat>
    <v-list-item-group>
      <template v-for='password in passwords'>
        <v-list-item :key='password.uuid'>
          <v-list-item-title>
            <v-chip :color='color'>
              {{ filter.split('_').join(' ') }}
            </v-chip>

            {{ password.name }}
            <span v-if='password.login'>
              - {{ password.login }}
            </span>
          </v-list-item-title>
        </v-list-item>

        <v-divider :key='password.uuid' />
      </template>
    </v-list-item-group>
  </v-list>
</template>

<script lang='ts'>
import Component, { mixins } from 'vue-class-component'
import { Prop } from 'vue-property-decorator'
import PasswordUtil from '~/mixins/PasswordUtil'

@Component({
  name: 'AnalyticsList'
})

export default class AnalyticsListComponent extends mixins(PasswordUtil) {
  @Prop({ required: true })
  filter!: string

  @Prop({ required: true, type: String })
  color!: string

  get passwords() {
    const passwords = this.$store.state.password.passwords
    const analytics = this.$store.state.password.analytics

    // make deep copy
    return [...passwords].filter((password: any) => analytics[this.filter].includes(password.uuid)
    )
  }
}
</script>
