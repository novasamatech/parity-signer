export function networkLogo(network: string): number | undefined {
	switch (network) {
		case 'polkadot':
			return require('res/img/logos/Polkadot.png');
		case 'kusama':
			return require('res/img/logos/Kusama.png');
		case 'rococo':
			return require('res/img/logos/Rococo.png');
		default:
			return;
	}
}
