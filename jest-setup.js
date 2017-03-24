/* global jest */
// import mockCamera from './__mocks__/Camera'

jest.unmock('Image')
jest.mock('react-native-camera', () => 'camera')
jest.mock('Linking', () => {
  return {
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    openURL: jest.fn(),
    canOpenURL: jest.fn(),
    getInitialURL: jest.fn()
  }
})
