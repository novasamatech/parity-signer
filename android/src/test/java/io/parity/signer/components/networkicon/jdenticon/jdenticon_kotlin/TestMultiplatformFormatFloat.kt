package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import org.junit.Assert.assertEquals
import org.junit.Test
import kotlin.math.PI

class TestMultiplatformFormatFloat {

    @Test
    fun testIntegerFormatting() {
        assertEquals("1.00", 1f.format(2))
        assertEquals("10.00", 10f.format(2))
        assertEquals("100.00", 100f.format(2))
    }

    @Test
    fun testTwoDigitsFormatting() {
        assertEquals("1.23", 1.23f.format(2))
        assertEquals("10.23", 10.23f.format(2))
        assertEquals("100.23", 100.23f.format(2))
    }

    @Test
    fun testRound() {
        assertEquals("3.14", PI.toFloat().format(2))
        assertEquals("0.00", 0.00001f.format(2))
    }

}
