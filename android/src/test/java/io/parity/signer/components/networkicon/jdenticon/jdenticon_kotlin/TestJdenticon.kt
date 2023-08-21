package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

import org.junit.Assert.assertEquals
import org.junit.Test

class TestJdenticon {

    private fun assertShouldBuildThisSvg(
        source: String,
        result: String
    ) {
        assertEquals(result, Jdenticon.toSvg(source, 100))
    }

    @Test
    fun `produces expected SVG output`() {
        assertShouldBuildThisSvg(
            "Alice",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#32995e" d="M0 0L25 0L25 25ZM100 0L100 25L75 25ZM100 100L75 100L75 75ZM0 100L0 75L25 75Z"/><path fill="#66cc91" d="M25 25L50 25L50 50L25 50ZM41 47L47 35L35 35ZM75 25L75 50L50 50L50 25ZM52 41L65 47L65 35ZM75 75L50 75L50 50L75 50ZM58 52L52 65L65 65ZM25 75L25 50L50 50L50 75ZM47 58L35 52L35 65Z"/><path fill="#e5e5e5" d="M29 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M54 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M54 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M29 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M4 37a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 37a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 62a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M4 62a8,8 0 1,0 16,0a8,8 0 1,0 -16,0"/></svg>"""
        )

        assertShouldBuildThisSvg(
            "Bob",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#a85b38" d="M37 25L25 12L37 0L50 12ZM50 12L62 0L75 12L62 25ZM62 75L75 87L62 100L50 87ZM50 87L37 100L25 87L37 75ZM12 50L0 37L12 25L25 37ZM75 37L87 25L100 37L87 50ZM87 50L100 62L87 75L75 62ZM25 62L12 75L0 62L12 50Z"/><path fill="#d19275" d="M25 25L50 25L50 50L25 50ZM34 40a6,6 0 1,0 13,0a6,6 0 1,0 -13,0M75 25L75 50L50 50L50 25ZM53 40a6,6 0 1,0 13,0a6,6 0 1,0 -13,0M75 75L50 75L50 50L75 50ZM53 59a6,6 0 1,0 13,0a6,6 0 1,0 -13,0M25 75L25 50L50 50L50 75ZM34 59a6,6 0 1,0 13,0a6,6 0 1,0 -13,0"/><path fill="#e5e5e5" d="M0 25L0 0L25 0ZM75 0L100 0L100 25ZM100 75L100 100L75 100ZM25 100L0 100L0 75Z"/></svg>"""
        )

        assertShouldBuildThisSvg(
            "deadbeef",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#729932" d="M4 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M4 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0"/><path fill="#a5cc66" d="M50 25L25 25L25 12ZM50 25L50 0L62 0ZM50 75L75 75L75 87ZM50 75L50 100L37 100ZM25 50L0 50L0 37ZM75 50L75 25L87 25ZM75 50L100 50L100 62ZM25 50L25 75L12 75Z"/><path fill="#d2e5b2" d="M35 41a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M53 41a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M53 59a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M35 59a6,6 0 1,0 12,0a6,6 0 1,0 -12,0"/></svg>"""
        )

        assertShouldBuildThisSvg(
            "deadbeef123",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#ab84d6" d="M50 12L37 25L25 12L37 0ZM62 25L50 12L62 0L75 12ZM50 87L62 75L75 87L62 100ZM37 75L50 87L37 100L25 87ZM25 37L12 50L0 37L12 25ZM87 50L75 37L87 25L100 37ZM75 62L87 50L100 62L87 75ZM12 50L25 62L12 75L0 62ZM25 25L50 25L50 29L39 50L25 50ZM75 25L75 50L71 50L50 39L50 25ZM75 75L50 75L50 71L60 50L75 50ZM25 75L25 50L29 50L50 60L50 75Z"/><path fill="#e5e5e5" d="M4 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M4 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0"/></svg>"""
        )

        assertShouldBuildThisSvg(
            "0123456789ABCDEF",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#3d6ab7" d="M0 25L0 0L25 0ZM75 0L100 0L100 25ZM100 75L100 100L75 100ZM25 100L0 100L0 75Z"/><path fill="#84a3d6" d="M50 25L50 45L38 25ZM75 50L55 50L75 38ZM50 75L50 55L62 75ZM25 50L45 50L25 62Z"/><path fill="#c1d1ea" d="M25 12L37 0L50 12L37 25ZM62 0L75 12L62 25L50 12ZM75 87L62 100L50 87L62 75ZM37 100L25 87L37 75L50 87ZM0 37L12 25L25 37L12 50ZM87 25L100 37L87 50L75 37ZM100 62L87 75L75 62L87 50ZM12 75L0 62L12 50L25 62Z"/></svg>"""
        )

        assertShouldBuildThisSvg(
            "f49cf6381e322b147053b74e4500af8533ac1e4c",
            """<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet"><path fill="#729932" d="M4 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 12a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M79 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0M4 87a8,8 0 1,0 16,0a8,8 0 1,0 -16,0"/><path fill="#a5cc66" d="M50 25L25 25L25 12ZM50 25L50 0L62 0ZM50 75L75 75L75 87ZM50 75L50 100L37 100ZM25 50L0 50L0 37ZM75 50L75 25L87 25ZM75 50L100 50L100 62ZM25 50L25 75L12 75Z"/><path fill="#d2e5b2" d="M35 41a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M53 41a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M53 59a6,6 0 1,0 12,0a6,6 0 1,0 -12,0M35 59a6,6 0 1,0 12,0a6,6 0 1,0 -12,0"/></svg>"""
        )
    }

}
