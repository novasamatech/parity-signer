package io.parity.signer.components.networkicon.dot

import io.parity.signer.components.networkicon.dot.DotIconColors.SchemeElement


internal object DotIconConstants {
	/// Function to set default coloring schemes, taken as is from js code
	fun defaultSchemes() = listOf(
			SchemeElement(
				// "target"
				freq =  1,
				colors = listOf(0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 1)
					.map { it.toByte() },
			),
			SchemeElement(
				// "cube",
				freq =  20,
				colors = listOf(0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 5)
					.map { it.toByte() },
			),
			SchemeElement(
				// "quazar",
				freq =  16,
				colors = listOf(1, 2, 3, 1, 2, 4, 5, 5, 4, 1, 2, 3, 1, 2, 4, 5, 5, 4, 0)
					.map { it.toByte() },
			),
			SchemeElement(
				// "flower",
				freq =  32,
				colors = listOf(0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 3)
					.map { it.toByte() },
			),
			SchemeElement(
				// "cyclic",
				freq =  32,
				colors = listOf(0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6)
					.map { it.toByte() },
			),
			SchemeElement(
				// "vmirror",
				freq =  128,
				colors = listOf(0, 1, 2, 3, 4, 5, 3, 4, 2, 0, 1, 6, 7, 8, 9, 7, 8, 6, 10)
					.map { it.toByte() },
			),
			SchemeElement(
				// "hmirror",
				freq =  128,
				colors = listOf(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 6, 7, 5, 3, 4, 2, 11)
					.map { it.toByte() },
			),
		)
}
