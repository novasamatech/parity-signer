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
  KUSAMA: 's2'
});

const substrateNetworkRaw = {
  [SubstrateNetworkKeys.KUSAMA]: {
    title: 'Kusama',
    ss58Prefix: 2,
    balanceModuleId: 123 // This id need to be checked
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

const SUBSTRATE_NETWORK_LIST = mapValues(
  substrateNetworkRaw,
  (substrateNetwork, substrateNetworkId) =>
    defaults(substrateNetwork, {
      protocol: NetworkProtocols.SUBSTRATE,
      color: '#4C4646',
      secondaryColor: colors.card_bg
    })
);

export const NETWORK_LIST = Object.freeze(
  Object.assign({}, ETHEREUM_NETWORK_LIST, SUBSTRATE_NETWORK_LIST)
);
