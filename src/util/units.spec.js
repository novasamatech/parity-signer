import { fromWei } from './units'

describe('units', () => {
  it('should properly convert units from wei', () => {
    let wei = '5208';
    let ether = fromWei(wei)
    expect(ether).toEqual("0.000000000000021")
  })
})
