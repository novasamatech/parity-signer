import sjcl from 'sjcl'
import words from '../../res/words.json'

export function random() {
  let count = 11
  let paranoia = 10
  let buf = sjcl.random.randomWords(count, paranoia)
  return buf.map(n => words[Math.abs(n) % words.length]).join(' ')
}
