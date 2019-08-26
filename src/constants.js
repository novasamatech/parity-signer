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
  // POLKADOT: '123',
  POLKADOT_TEST: '0xe4e7807c233645b910c8db58e99ed53dc71fbfff5bbe8a5534fb7e83db449210', // genesis hash from v0.5.1 polkadot --dev commit e086465916c2778a7dede7dc62e551d801dc12ca
});

const substrateNetworkBase = {
  [SubstrateNetworkKeys.KUSAMA]: {
    color: '#4C4646',
    genesisHash: SubstrateNetworkKeys.KUSAMA,
    prefix: 2,
    title: 'Kusama'
  },
  [SubstrateNetworkKeys.POLKADOT_TEST]: {
    color: '#ff8c00',
    genesisHash: SubstrateNetworkKeys.POLKADOT_TEST,
    prefix: 42,
    title: 'Polkadot Dev'
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