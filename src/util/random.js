import dictionary from '../../res/words.json'

// TODO: make it secure!
export function words() {
  let buf = (new Uint16Array(11)).map(_ => Math.random() * dictionary.length)
  return Array.from(buf).map(n => dictionary[n]).join(' ')
}

