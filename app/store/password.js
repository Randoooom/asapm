/*
 * MIT LICENSE
 *
 * Copyright (c) 2022 Randoooom
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

import { invoke } from '@tauri-apps/api/tauri'

export const state = () => ({
  currentPassword: undefined,
  passwords: [],
  analytics: undefined
})

export const mutations = {
  /**
   * update the currentPassword
   * @param state
   * @param password
   */
  pushPassword(state, password) {
    state.currentPassword = password
  },

  /**
   * set the password list
   * @param state
   * @param passwords
   */
  setPasswords(state, passwords) {
    state.passwords = passwords
  },

  /**
   * set the password analytics
   * @param state
   * @param stats
   */

  setAnalytics(state, analytics) {
    state.analytics = analytics
  }
}

export const actions = {
  /**
   * fetch the data from tauri
   */
  async fetchData({ commit }) {
    // invoke command
    await invoke('get_passwords')
      .then((passwords) => commit('setPasswords', passwords))

    // also handle the generator
    await invoke('get_generator')
      .then((generator) => commit('generator/setDefault', generator, { root: true }))

    // get the analytics
    await invoke('analyse')
      .then((result) => commit('setAnalytics', result))
  },
}
