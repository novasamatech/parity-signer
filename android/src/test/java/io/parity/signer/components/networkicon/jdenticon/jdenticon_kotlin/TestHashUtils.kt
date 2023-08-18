package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import org.junit.Assert.assertEquals
import org.junit.Test

class TestHashUtils {

    @Test
    fun `check hex string candidate regex correctness`() {

        // these fail to match
        assertEquals(false, HashUtils.isValidHash(""))
        assertEquals(false, HashUtils.isValidHash("0"))
        assertEquals(false, HashUtils.isValidHash("1"))
        assertEquals(false, HashUtils.isValidHash("A"))
        assertEquals(false, HashUtils.isValidHash("f"))
        assertEquals(false, HashUtils.isValidHash("0123456789"))
        assertEquals(false, HashUtils.isValidHash("01234abdce"))
        assertEquals(false, HashUtils.isValidHash("56789FEDCB"))
        assertEquals(false, HashUtils.isValidHash("1122334455ZZYYXX"))

        // these are valid matches
        assertEquals(true, HashUtils.isValidHash("0123456789A"))
        assertEquals(true, HashUtils.isValidHash("0123456789ABCDEF"))
        assertEquals(true, HashUtils.isValidHash("0123456789abcdef"))
    }

    @Test
    fun `keeps or creates correct hash`() {
        assertEquals("da39a3ee5e6b4b0d3255bfef95601890afd80709", HashUtils.keepOrCreateHash(""))
        assertEquals("35318264c9a98faf79965c270ac80c5606774df1", HashUtils.keepOrCreateHash("Alice"))
        assertEquals("da6645f6e22bf5f75974dc7eed5fcd6160d6b51e", HashUtils.keepOrCreateHash("Bob"))
        assertEquals("f49cf6381e322b147053b74e4500af8533ac1e4c", HashUtils.keepOrCreateHash("deadbeef"))
        assertEquals("deadbeef123", HashUtils.keepOrCreateHash("deadbeef123"))
        assertEquals(
            "f49cf6381e322b147053b74e4500af8533ac1e4c",
            HashUtils.keepOrCreateHash("f49cf6381e322b147053b74e4500af8533ac1e4c")
        )
        assertEquals("0123456789ABCDEF", HashUtils.keepOrCreateHash("0123456789ABCDEF"))
        assertEquals("0123456789abcdef", HashUtils.keepOrCreateHash("0123456789abcdef"))
    }

}
