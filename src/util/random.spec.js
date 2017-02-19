import { words } from './random'

describe('words', () => {
  it('should create a list of 10 random words', () => {
    let randomWords = words()
    let count = randomWords.split(' ').length
    expect(count).toEqual(11)
  })
})
