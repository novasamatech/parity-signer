export function isAscii(data) {
	return /^[\x00-\x7F]*$/.test(data);
}

export function hexToAscii(hexx) {
	const hex = hexx.toString();
	let str = '';
	for (let i = 0; i < hex.length; i += 2) {
		str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
	}

	return str;
}
