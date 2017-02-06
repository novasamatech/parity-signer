import React from 'react';
import 'react-native'
import App from '../src/components/App'

// Note: test renderer must be required after react-native.
import renderer from 'react-test-renderer';

it('renders correctly', () => {
  const tree = renderer.create(
    <App/>
  )
});
