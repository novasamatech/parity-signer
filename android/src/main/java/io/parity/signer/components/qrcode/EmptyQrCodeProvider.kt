package io.parity.signer.components.qrcode


class EmptyQrCodeProvider : AnimatedQrKeysProvider<List<List<UByte>>> {

	override suspend fun getQrCodesList(input: List<List<UByte>>): List<List<UByte>> {
		return input
	}
}

