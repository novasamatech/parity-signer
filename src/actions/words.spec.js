import { random } from './words'

describe('words', () => {
  it('should create a list of 10 random words', () => {
    let randomWords = random()
    let count = randomWords.split(' ').length
    expect(count).toEqual(11)
  })
})
