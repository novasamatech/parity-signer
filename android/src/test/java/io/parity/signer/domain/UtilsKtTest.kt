package io.parity.signer.domain

import org.junit.Assert
import org.junit.Test

class UtilsKtTest {
	@Test
	fun encodeHex() {
		Assert.assertEquals( "%02x".format(0.toByte()), byteArrayOf(0.toByte()).encodeHex())
	}
}


