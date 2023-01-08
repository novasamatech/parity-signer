package io.parity.signer.screens.keyderivation

import org.junit.Test
import org.junit.Assert.*


class DerivationPathAnalyzerTest {

	@Test
	fun testHint() {
		val analyzer = DerivationPathAnalyzer()
		assertEquals(DerivationPathAnalyzer.Hint.CREATE_PASSWORD, analyzer.getHint("//seed///"))
		assertEquals(DerivationPathAnalyzer.Hint.PATH_NAME, analyzer.getHint("//seed//"))
		assertEquals(DerivationPathAnalyzer.Hint.PATH_NAME, analyzer.getHint("//"))
		assertEquals(DerivationPathAnalyzer.Hint.NONE, analyzer.getHint("//seed///sd"))
		assertEquals(DerivationPathAnalyzer.Hint.NONE, analyzer.getHint("//se"))
	}

	@Test
	fun testHidePassword() {
		val analyzer = DerivationPathAnalyzer()
		assertEquals("//seed///", analyzer.hidePasswords("//seed///"))
		assertEquals("//seed///••••", analyzer.hidePasswords("//seed///dfgf"))
		assertEquals("//seed///••••///••••", analyzer.hidePasswords("//seed///sdfg///hjkj"))
		assertEquals("//seed", analyzer.hidePasswords("//seed"))

	}
}
