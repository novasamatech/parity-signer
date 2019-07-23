import mapValues from 'lodash/mapValues';
import defaults from 'lodash/defaults';
import colors from './colors';

export const NetworkTypes = Object.freeze({
  ETHEREUM: 'ethereum',
  SUBSTRATE: 'substrate'
});

export const EthereumNetworkIds = Object.freeze({
  OLYMPIC: '0',
  FRONTIER: '1',
  EXPANSE: '2',
  ROPSTEN: '3',
  RINKEBY: '4',
  GOERLI: '5',
  UBIG: '8',
  KOVAN: '42',
  CLASSIC: '61',
  SOKOL: '77',
  CORE: '99',
  MUSICOIN: '7762959'
});

export const SubstrateNetworkIds = Object.freeze({
  KUSAMA: 's0'
});

const substrateNetworkRaw = {
  [SubstrateNetworkIds.KUSAMA]: {
    name: 'kusama',
    ss58Prefix: 2,
    balanceModuleId: 123 // This id need to be checked
  }
};

const ethereumNetworkRaw = {
  [EthereumNetworkIds.OLYMPIC]: {},
  [EthereumNetworkIds.FRONTIER]: {
    title: 'Ethereum',
    color: '#977CF6',
    secondaryColor: colors.card_bg,
    available: true
  },
  [EthereumNetworkIds.CLASSIC]: {
    title: 'Ethereum Classic',
    color: '#8C7166',
    secondaryColor: colors.card_bg,
    available: true
  },
  [EthereumNetworkIds.EXPANSE]: {
    title: 'Expanse'
  },
  [EthereumNetworkIds.ROPSTEN]: {
    title: 'Ropsten Testnet',
    available: true
  },
  [EthereumNetworkIds.RINKEBY]: {
    title: 'Rinkeby Testnet'
  },
  [EthereumNetworkIds.GOERLI]: {
    title: 'Görli Testnet',
    available: true
  },
  [EthereumNetworkIds.KOVAN]: {
    title: 'Kovan Testnet',
    available: true
  },
  [EthereumNetworkIds.SOKOL]: {},
  [EthereumNetworkIds.CORE]: {},
  [EthereumNetworkIds.MUSICOIN]: {}
};

export const ETHEREUM_NETWORK_LIST = mapValues(
  ethereumNetworkRaw,
  (ethereumNetwork, ethereumChainId) =>
    defaults(ethereumNetwork, {
      protocol: NetworkTypes.ETHEREUM,
      color: '#F2E265',
      secondaryColor: colors.card_text,
      available: false,
      title: `Ethereum_${ethereumChainId}`,
      ethereumChainId: ethereumChainId
    })
);

const SUBSTRATE_NETWORK_LIST = mapValues(
  substrateNetworkRaw,
  (substrateNetwork, substrateNetworkId) =>
    defaults(substrateNetwork, {
      protocol: NetworkTypes.SUBSTRATE,
      color: '#E6007A',
      secondaryColor: colors.card_bg,
      available: false
    })
);

export const NETWORK_LIST = Object.freeze(
  Object.assign({}, ETHEREUM_NETWORK_LIST, SUBSTRATE_NETWORK_LIST)
);
