import React from 'react';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';

export const withAccountStore = WrappedComponent => {
	return () => (
		<Subscribe to={[AccountsStore]}>
			{accounts => <WrappedComponent {...this.props} accounts={accounts} />}
		</Subscribe>
	);
};
