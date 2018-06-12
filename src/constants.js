export const NETWORK_TYPE = {
  ethereum: 'ethereum'
};

export const NETWORK_IDS = {
  '0': 'olympic',
  '1': 'frontier',
  '2': 'expanse',
  '3': 'ropsten',
  '4': 'rinkeby',
  '8': 'ubiq',
  '42': 'kovan',
  '61': 'classic',
  '77': 'sokol',
  '99': 'core',
  '7762959': 'musicoin'
};

export const NETWORK_ID = Object.entries(NETWORK_IDS).reduce(
  (acc, [key, value]) => Object.assign(acc, { [value]: key }),
  {}
);

export const NETWORK_LIST = ['1', '61', '3', '42'];

export const NETWORK_TITLES = {
  [NETWORK_ID.frontier]: 'Ethereum',
  [NETWORK_ID.classic]: 'Ethereum Classic',
  [NETWORK_ID.ropsten]: 'Ropsten Testnet',
  [NETWORK_ID.kovan]: 'Kovan Testnet'
};

export const NETWORK_COLOR = {
  [NETWORK_ID.frontier]: '#977CF6',
  [NETWORK_ID.classic]: '#FC2166'
};

export const DEFAULT_NETWORK_COLOR = '#F2E265';
