package io.parity.signer.models

fun String.decodeHex(): ByteArray {
	return chunked(2).map { it.toInt(16).toByte() }.toByteArray()
}

class Utils {
}
