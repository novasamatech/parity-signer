import dictionary from '../../res/words.json'

// TODO: make it secure!
export function words () {
  let buf = Array.apply(null, { length: 11 }).map(Function.call, () => Math.random() * dictionary.length)
  return buf.map(n => dictionary[Math.floor(n)]).join(' ')
}
