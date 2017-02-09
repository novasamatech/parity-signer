import { ec } from 'elliptic'
import { keccak_256 } from 'js-sha3'
var secp = new ec('secp256k1')

export function keypairFromPhrase(phrase) {
	var seed = keccak_256.array(phrase);
	var kp;
	for (var i = 0; i <= 16384 || !toAddress(kp = secp.keyFromPrivate(seed)).startsWith('00'); ++i)
		seed = keccak_256.array(seed)
	return kp
}

export function toAddress(kp) { return keccak_256(kp.getPublic('buffer').slice(1)).substr(24); }

