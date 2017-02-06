import dictionary from '../../res/words.json'

// TODO: make it secure!
export function words() {
  let buf = (new Uint16Array(11)).map(_ => Math.random() * dictionary.length)
  return Array.from(buf).map(n => dictionary[n]).join(' ')
}

// this should be later replaced with random keypair
export function address() {
  return '0xb794f5ea0ba39494ce839613fffba74279579268'
}
