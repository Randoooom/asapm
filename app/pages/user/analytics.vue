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
        Password analytics
      </v-card-title>

      <v-card-text class='mt-5'>
        <v-row>
          <v-col v-for='(key, index) in Object.keys(stats)' :key='index' cols='12' sm='4' md='4' class='analytics'>
            <div>
              {{ key.split('_').join(' ').toUpperCase() }}
            </div>
            <span class='text-h4 primary--text'>
              {{ stats[key].length }}
            </span>
          </v-col>
        </v-row>
      </v-card-text>
    </v-card>

    <v-card class='mt-10'>
      <v-card-subtitle>
        Risky passwords
      </v-card-subtitle>

      <v-card-text>
        <v-tabs v-model='tab'>
          <v-tab>
            Reused
          </v-tab>

          <v-tab>
            Medium
          </v-tab>

          <v-tab>
            Weak
          </v-tab>

          <v-tab>
            Very weak
          </v-tab>
        </v-tabs>

        <v-tabs-items v-model='tab'>
          <v-tab-item>
            <analytics-list color='warning' filter='reused' />
          </v-tab-item>

          <v-tab-item>
            <analytics-list color='warning' filter='medium' />
          </v-tab-item>

          <v-tab-item>
            <analytics-list color='red' filter='weak' />
          </v-tab-item>

          <v-tab-item>
            <analytics-list color='error' filter='very_weak' />
          </v-tab-item>
        </v-tabs-items>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'

@Component({
  name: 'Analytics',
  components: {
    'analytics-list': () => import('~/components/password/AnalyticsList.vue')
  }
})

export default class AnalyticsComponent extends Vue {
  tab = null

  get stats() {
    return this.$store.state.password.analytics
  }
}
</script>

<style lang='sass' scoped>
.analytics
  padding-bottom: 20px

  *
    text-align: center
    width: 100%
    display: block
</style>
