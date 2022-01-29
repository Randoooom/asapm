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

import Vue from 'vue'
import Component from 'vue-class-component'
import { invoke } from '@tauri-apps/api/tauri'

@Component
export default class PasswordUtil extends Vue {
  /**
   * copy the selected password into the clipboard
   * @param password
   */
  async copyPassword(password: any) {
    // copy the password
    await this.$copyText(password.password)
      .then(async () => await this.$store.commit('snackbar/emitSnackbar', {
        color: 'success',
        text: 'Password copied',
        outlined: true
      }))
      // handle empty password
      .catch(async () => await this.$store.commit('snackbar/emitSnackbar', {
        color: 'error',
        text: 'Password is empty'
      }))
  }

  /**
   * generate new password from the generator
   */
  async generatePassword(generator: { length: number, symbols: boolean, letters: boolean, numbers: boolean } | null = null): Promise<string> {
    return await invoke('generate_password', { generator })
      .then(value => value as string)
  }

  /**
   * get the strength of the given password
   * @param password
   */
  async getPasswordStrength(password: String): Promise<string> {
    if (password.length === 0) return 'Blank'

    switch (await invoke('password_strength', { password }).then(score => score as number)) {
      case 0:
        return 'Very weak'
      case 1:
        return 'Weak'
      case 2:
        return 'Medium'
      case 3:
        return 'Strong'
      case 4:
        return 'Very strong'
      default:
        return 'Error'
    }
  }
}

