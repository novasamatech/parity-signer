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
  SUBSTRATE: 'substrate'
});

export const SubstratePrefixKeys = Object.freeze({
  KUSAMA: 'kusama',
  POLKADOT: 'polkadot',
});

export const SubstratePrefixes = Object.freeze({
  [SubstratePrefixKeys.KUSAMA]: {
    prefix: 2,
    color: '#1e1e1e'
  },
  [SubstratePrefixKeys.POLKADOT]: {
    prefix: 0,
    color: '#e6007a'
  }
});

const SUBSTRATE_NETWORK_LIST = {
  [SubstrateNetworkKeys.SUBSTRATE]: {
    title: 'Substrate',
    protocol: NetworkProtocols.SUBSTRATE,
    color: '#4C4646',
    secondaryColor: colors.card_bg
  }
};

const ethereumNetworkRaw = {
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
  ethereumNetworkRaw,
  (ethereumNetwork, ethereumChainId) =>
    defaults(ethereumNetwork, {
      protocol: NetworkProtocols.ETHEREUM,
      color: '#F2E265',
      secondaryColor: colors.card_text,
      ethereumChainId: ethereumChainId
    })
);

export const NETWORK_LIST = Object.freeze(
  Object.assign({}, SUBSTRATE_NETWORK_LIST, ETHEREUM_NETWORK_LIST)
);
