//DEPRECATED

import hoistNonReactStatic from 'hoist-non-react-statics';
import * as React from 'react';
import { Dimensions } from 'react-native';

export const isOrientationLandscape = ({ height, width }) => width > height;

export default function withDimensions(WrappedComponent) {
	const { height, width } = Dimensions.get('window');

	class EnhancedComponent extends React.Component {
		static displayName = `withDimensions(${WrappedComponent.displayName})`;

		state = {
			dimensions: { height, width },
			isLandscape: isOrientationLandscape({ height, width })
		};

		componentDidMount() {
			Dimensions.addEventListener('change', this.handleOrientationChange);
		}

		componentWillUnmount() {
			Dimensions.removeEventListener('change', this.handleOrientationChange);
		}

		handleOrientationChange = ({ window }) => {
			const isLandscape = isOrientationLandscape(window);

			this.setState({ isLandscape });
		};

		render() {
			return <WrappedComponent {...this.props}
				{...this.state} />;
		}
	}

	return hoistNonReactStatic(EnhancedComponent, WrappedComponent);
}
