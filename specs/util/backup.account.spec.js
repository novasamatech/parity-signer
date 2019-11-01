/* eslint-disable */
import { emptyAccount } from '../../src/util/account';

const testAccount1 = {
	address: 'HemEdUAn7YydoU56DZfHzrDQPNCmVJP7mfzzwi7ib6ysQVQ',
	createdAt: 1571683693196,
	encryptedSeed:
		'{"cipher":"aes-128-ctr","cipherparams":{"iv":"7f2e26a8614f7736ea4242f7a44db5fb"},"ciphertext":"1457dc131c21658d54972259dd6d9a4aa6fa011ccce7ad2512bb49cd0957a68004ff347816307f67d3ad886f243bb409f6c18aa43c38033fe542c9d0e109454e9134f5e8210fdf5fcf9a334b505702f215aebdf35f0faa423d27e09ecf6311e9f582075b7a39b32c24a96564d25490278c7fd446a89911f878602f2d9cddc69e1674f6a7fec0b9d773538fdbcd8b863cca7f59149dde122f","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"6f25d822334adacc88e0560eec1d3c530c62719a87c038dd2c179aad2ef145c7"},"mac":"69c2247fc2fc28cb733ae732fe693530c0dbcb56eeea223e243c2dc2bf311ddc"}',
	name: 'AccountSubstrate',
	networkKey:
		'0xe3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636',
	updatedAt: 1571683716847,
	validBip39Seed: true
};

const testAccount2 = {
	address: 'EiDA4ysAUks2K1wprqFEKiJVi1J8WxtBKRv4T1NT6Y5qbb7',
	createdAt: 1571683716907,
	encryptedSeed:
		'{"cipher":"aes-128-ctr","cipherparams":{"iv":"d66176154671ce1d696f5a3866452edb"},"ciphertext":"75af9aef2b673513ad7445186ad71cb2d32762d7b6f7d17d43d200a6c28030591b828632a94cbf5acf64b1e60d17ae058a8e9becc9f362dd7a5842414606911b20774eb56fab66c839e44bf3a7afe8d11edff566cc108871a1f004b0c27a6a3e240e22b31a9f78440752db504f468961ac8b327d31ec16a8d6eb69c83c3939f1a91b5a0d0328be5a1cd46306433029e023d6e691142d02","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"ee607febcff48ce6f5cfcd0ba79aed3978593a9762bb67ac8511339f8624ec5e"},"mac":"556e95841edfeed98f8d182658894e6278adcc59b5fccc30b6b0d383f51e63c9"}',
	name: 'Substrate2',
	networkKey:
		'0xe3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636',
	updatedAt: 1571683731544,
	validBip39Seed: true
};

const testAccount3 = {
	address: '330C9C7a267c1F482f764EEfEA1de5cE945D6D79',
	createdAt: 1571683737847,
	encryptedSeed:
		'{"cipher":"aes-128-ctr","cipherparams":{"iv":"b68afbaf7627ff75c9e998da8853a0fd"},"ciphertext":"0fdf9029a0a913f4b510b33370d952bb6c06649ba56ca84cd6fbd376ab1025f173330d2a410f69b1104dad88b890c82c634f81ba9b33a1e29eb6b559cd62a05805621147e2974b075dcbde9a957bab1614bd0badefc810bc6c1bc7d75f6af56d693c4a6fb32f7da40df33436c8faf6e47373687c6f27822ba35060b337658355e110ea2dad3df5306ae3a12276cccd2e","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"9aec45e498cb9acef73f1764fa4439dca8b9785414f6117cec5dbae35d2eeb11"},"mac":"a45d2874ca4221fd46d7481dee3e98b532eb7dd6deaac6bb2de1b1895263ea6b"}',
	name: 'Ethereum1',
	networkKey: '1',
	updatedAt: 1571683749227,
	validBip39Seed: true
};

const testAccount4WithPath = {
	address: '5Cm57mKuh5ZUjRVJEY67oNG26iNzFfy3XZYzNgwxhpBTPnLZ',
	createdAt: 1571684189994,
	encryptedSeed:
		'{"cipher":"aes-128-ctr","cipherparams":{"iv":"a4ea4bcf1210a78ee53871e914eb75da"},"ciphertext":"ee9f16a0e1b68bec082c3a1b8411385c0d4d47082da5ba0ec7d05186235a1620f1e79dc67decdda41aa79db4b3a9657f720addd8b1abd3b022e68b25381f29808939b87e0c63a83a14bf890578e9a2cdf5ec6c9b373699dbac5d4f47d3b084d7e6ecc588e724a138346885efc961df33ad5a57187981a57d78d73b8dc09e0f1ee3fe9813f697cd1ea6a31506b2603dbf6d5a845c409cabd47b44ec9658679130f5ccae0d8e737675","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"fae3012b00edc3c3604d2a1b03d14c37f51d57d44d817f5ae49de42f441757b4"},"mac":"0e424eadb5eb83c440a0b70102992e06e07ac1f5c0038cc2c84c64daee4c0686"}',
	name: 'DevelopAccount',
	networkKey:
		'0x4393a679e1830a487e8ae92733f089a80f3e24ba515b08dd8adb40fc6cedee8d',
	updatedAt: 1571684212852,
	validBip39Seed: true
};

const accountsMap = new Map();
accountsMap.set(
	'substrate:HemEdUAn7YydoU56DZfHzrDQPNCmVJP7mfzzwi7ib6ysQVQ:0xe3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636',
	testAccount1
);
accountsMap.set(
	'substrate:EiDA4ysAUks2K1wprqFEKiJVi1J8WxtBKRv4T1NT6Y5qbb7:0xe3777fa922cafbff200cadeaea1a76bd7898ad5b89f7848999058b50e715f636',
	testAccount2
);
accountsMap.set(
	'ethereum:0x330c9c7a267c1f482f764eefea1de5ce945d6d79@1',
	testAccount3
);
accountsMap.set(
	'substrate:5Cm57mKuh5ZUjRVJEY67oNG26iNzFfy3XZYzNgwxhpBTPnLZ:0x4393a679e1830a487e8ae92733f089a80f3e24ba515b08dd8adb40fc6cedee8d',
	testAccount4WithPath
);

const sampleV3Account = {
	accounts: accountsMap,
	newAccount: emptyAccount(),
	selectedKey: ''
};

const unlockedAccountSample = {
	...testAccount4WithPath,
	...{
		derivationPath: '//good',
		derivationPassword: '111',
		seed: '',
		seedPhrase: ''
	}
};

function parseAccounts(value, key, map) {
	const {
		address,
		createdAt,
		encryptedSeed,
		name,
		netWorkKey,
		updatedAt,
		derivationPath,
		derivationPassword
	} = value;

	return {
		encryptedSeed,
		derivationPassword,
		addresses: new Map(),
		meta: new Map()
	};
}

const migrateAccount = V3AccountStore => {
	V3AccountStore.accounts.forEach();
};
