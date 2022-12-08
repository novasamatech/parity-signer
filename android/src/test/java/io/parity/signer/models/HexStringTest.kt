package io.parity.signer.models

import io.parity.signer.ui.helpers.PreviewData
import org.junit.Test


class HexStringTest {
	fun String.decodeHex(): ByteArray {
		check(length % 2 == 0) { "Must have an even length" }

		val byteIterator = chunkedSequence(2)
			.map { it.toInt(16).toByte() }
			.iterator()

		return ByteArray(length / 2) { byteIterator.next() }
	}

	@Test
	fun see() {
		val hex = PreviewData.exampleMarkdownDocs
		val bytes = hex.decodeHex()
		println(bytes)
		println(String(bytes))
		println("Some ")

	}
}
