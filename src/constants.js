import colors from './colors';

export const NetworkProtocols = Object.freeze({
  ETHEREUM: 'ethereum',
  SUBSTRATE: 'substrate'
});

// ethereumChainId is used as Network key for Ethereum networks
export const EthereumNetworkKeys = Object.freeze({
  FRONTIER: '1',
  ROPSTEN: '3',
  RINKEBY: '4',
  GOERLI: '5',
  KOVAN: '42',
  CLASSIC: '61',
});

// genesisHash is used as Network key for Substrate networks
export const SubstrateNetworkKeys = Object.freeze({
  KUSAMA: '0x3fd7b9eb6a00376e5be61f01abb429ffb0b104be05eaff4d458da48fcd425baf', // https://polkascan.io/pre/kusama/block/0
  SUBSTRATE_DEV: '0x4393a679e1830a487e8ae92733f089a80f3e24ba515b08dd8adb40fc6cedee8d', // substrate --dev commit ac6a2a783f0e1f4a814cf2add40275730cd41be1 hosted on wss://dev-node.substrate.dev .
});

const substrateNetworkBase = {
  [SubstrateNetworkKeys.KUSAMA]: {
    color: '#4C4646',
    genesisHash: SubstrateNetworkKeys.KUSAMA,
    prefix: 2,
    title: 'Kusama'
  },
  [SubstrateNetworkKeys.SUBSTRATE_DEV]: {
    color: '#ff8c00',
    genesisHash: SubstrateNetworkKeys.SUBSTRATE_DEV,
    prefix: 42,
    title: 'Substrate Development'
  },
  // [SubstrateNetworkKeys.POLKADOT]: {
  //   color: '#e6007a',
  //   genesisHash: SubstrateNetworkKeys.POLKADOT,
  //   prefix: 0,
  //   title: 'Polkadot mainnet'
  // }
};

const ethereumNetworkBase = {
  [EthereumNetworkKeys.FRONTIER]: {
    color: '#977CF6',
    ethereumChainId: EthereumNetworkKeys.FRONTIER,
    secondaryColor: colors.card_bg,
    title: 'Ethereum'
  },
  [EthereumNetworkKeys.CLASSIC]: {
    color: '#8C7166',
    ethereumChainId: EthereumNetworkKeys.CLASSIC,
    secondaryColor: colors.card_bg,
    title: 'Ethereum Classic'
  },
  [EthereumNetworkKeys.ROPSTEN]: {
    ethereumChainId: EthereumNetworkKeys.ROPSTEN,
    title: 'Ropsten Testnet'
  },
  [EthereumNetworkKeys.GOERLI]: {
    ethereumChainId: EthereumNetworkKeys.GOERLI,
    title: 'Görli Testnet'
  },
  [EthereumNetworkKeys.KOVAN]: {
    ethereumChainId: EthereumNetworkKeys.KOVAN,
    title: 'Kovan Testnet'
  }
};

const ethereumDefaultValues = {
  color: '#F2E265',
  protocol: NetworkProtocols.ETHEREUM,
  secondaryColor: colors.card_text
};

const substrateDefaultValues = {
  color: '#4C4646',
  protocol: NetworkProtocols.SUBSTRATE,
  secondaryColor: colors.card_bg
}

function setDefault(networkBase, defaultProps) {
  return Object.keys(networkBase).reduce((acc,networkKey) => {
      return {
        ...acc,
        [networkKey]: {
          ...defaultProps,
          ...networkBase[networkKey]
        }
      }
    },{})
}

export const ETHEREUM_NETWORK_LIST = Object.freeze(setDefault(ethereumNetworkBase, ethereumDefaultValues));
export const SUBSTRATE_NETWORK_LIST = Object.freeze(setDefault(substrateNetworkBase, substrateDefaultValues));
export const NETWORK_LIST = Object.freeze(
  Object.assign({}, SUBSTRATE_NETWORK_LIST, ETHEREUM_NETWORK_LIST)
);

export const TX_DETAILS_MSG = "After signing and publishing you will have sent";