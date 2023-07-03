package io.parity.signer.screens.createderivation

import org.junit.Test
import org.junit.Assert.*


class DerivationPathAnalyzerTest {

	@Test
	fun testHint() {
		val analyzer = DerivationPathAnalyzer()
		assertEquals(DerivationPathAnalyzer.Hint.CREATE_PASSWORD, analyzer.getHint("//seed///"))
		assertEquals(DerivationPathAnalyzer.Hint.CREATE_PASSWORD, analyzer.getHint("//seed//1///"))
		assertEquals(DerivationPathAnalyzer.Hint.PATH_NAME, analyzer.getHint("//seed//"))
		assertEquals(DerivationPathAnalyzer.Hint.PATH_NAME, analyzer.getHint("//"))
		assertEquals(DerivationPathAnalyzer.Hint.NONE, analyzer.getHint("//seed///sd"))
		assertEquals(DerivationPathAnalyzer.Hint.NONE, analyzer.getHint("//se"))
		assertEquals(DerivationPathAnalyzer.Hint.CREATING_ROOT, analyzer.getHint(""))
	}

	@Test
	fun testHidePassword() {
		val analyzer = DerivationPathAnalyzer()
		assertEquals("//seed///", DerivationPathAnalyzer.hidePasswords("//seed///"))
		assertEquals("//seed///••••", DerivationPathAnalyzer.hidePasswords("//seed///dfgf"))
		assertEquals("//seed//1///••••", DerivationPathAnalyzer.hidePasswords("//seed//1///dfgf"))
		assertEquals("//seed///•••••••••••", DerivationPathAnalyzer.hidePasswords("//seed///sdfg///hjkj"))
		assertEquals("//seed", DerivationPathAnalyzer.hidePasswords("//seed"))
	}

	@Test
	fun gettingPassword() {
		val analyzer = DerivationPathAnalyzer()
		assertEquals("", DerivationPathAnalyzer.getPassword("//seed///"))
		assertEquals("gg", DerivationPathAnalyzer.getPassword("//seed///gg"))
		assertEquals("gg", DerivationPathAnalyzer.getPassword("//seed//1///gg"))
		assertEquals("/gg", DerivationPathAnalyzer.getPassword("//seed//1////gg"))
		assertEquals("/gg///ss", DerivationPathAnalyzer.getPassword("//seed//1////gg///ss"))

		assertNull(DerivationPathAnalyzer.getPassword("//seed//"))
		assertNull(DerivationPathAnalyzer.getPassword("//"))
		assertNull(DerivationPathAnalyzer.getPassword(""))
		assertNull(DerivationPathAnalyzer.getPassword("//seed//sdfsdf//"))
	}

	@Test
	fun checkIsCorrect() {
		val analyzer = DerivationPathAnalyzer()
		assertTrue(analyzer.isCorrect("//seed"))
		assertTrue(analyzer.isCorrect("//seed///sdfsdf"))
		assertTrue(analyzer.isCorrect("//seed//sdfsdf"))
		assertTrue(analyzer.isCorrect("/asdd"))
		assertTrue(analyzer.isCorrect("///")) // no password is another error - not correctness
		assertTrue(analyzer.isCorrect("//seed///")) // no password is another error
		assertTrue(analyzer.isCorrect("///sdf"))

		assertFalse(analyzer.isCorrect("//"))
		assertTrue(analyzer.isCorrect(""))//root key
	}
}
