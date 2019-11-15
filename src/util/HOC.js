import React from 'react';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';

export const withAccountStore = WrappedComponent => {
	return () => (
		<Subscribe to={[AccountsStore]}>
			{accounts => <WrappedComponent {...this.props} accounts={accounts} />}
		</Subscribe>
	);
};

export const withScannerStore = WrappedComponent => {
	return () => (
		<Subscribe to={[ScannerStore]}>
			{scanner => <WrappedComponent {...this.props} scanner={scanner} />}
		</Subscribe>
	);
};
