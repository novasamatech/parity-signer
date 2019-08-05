import mapValues from 'lodash/mapValues';
import defaults from 'lodash/defaults';
import colors from './colors';

export const NetworkProtocols = Object.freeze({
  ETHEREUM: 'ethereum',
  SUBSTRATE: 'substrate'
});

export const EthereumNetworkKeys = Object.freeze({
  FRONTIER: '1',
  ROPSTEN: '3',
  RINKEBY: '4',
  GOERLI: '5',
  KOVAN: '42',
  CLASSIC: '61',
});

export const SubstrateNetworkKeys = Object.freeze({
  POLKADOT: 's0',
  KUSAMA: 's2'
});

const substrateNetworkBase = {
  [SubstrateNetworkKeys.KUSAMA]: {
    color: '#4C4646',
    genesisHash: 0x123,
    prefix: 2,
    title: 'Kusama'
  },
  [SubstrateNetworkKeys.POLKADOT]: {
    color: '#e6007a',
    genesisHash: 0x456,
    prefix: 0,
    title: 'Polkadot'
  }
};

const ethereumNetworkBase = {
  [EthereumNetworkKeys.FRONTIER]: {
    title: 'Ethereum',
    color: '#977CF6',
    secondaryColor: colors.card_bg,
  },
  [EthereumNetworkKeys.CLASSIC]: {
    title: 'Ethereum Classic',
    color: '#8C7166',
    secondaryColor: colors.card_bg,
  },
  [EthereumNetworkKeys.ROPSTEN]: {
    title: 'Ropsten Testnet',
  },
  [EthereumNetworkKeys.GOERLI]: {
    title: 'Görli Testnet',
  },
  [EthereumNetworkKeys.KOVAN]: {
    title: 'Kovan Testnet',
  }
};

export const ETHEREUM_NETWORK_LIST = mapValues(
  ethereumNetworkBase,
  (ethereumNetworkKey, ethereumChainId) =>
    defaults(ethereumNetworkKey, {
      protocol: NetworkProtocols.ETHEREUM,
      color: '#F2E265',
      secondaryColor: colors.card_text,
      ethereumChainId
    })
);

export const SUBSTRATE_NETWORK_LIST = mapValues(
  substrateNetworkBase,
  (substrateNetworkKey) =>
    defaults(substrateNetworkKey, {
      color: '#4C4646',
      protocol: NetworkProtocols.SUBSTRATE,
      secondaryColor: colors.card_bg,
    })
);

export const NETWORK_LIST = Object.freeze(
  Object.assign({}, SUBSTRATE_NETWORK_LIST, ETHEREUM_NETWORK_LIST)
);
