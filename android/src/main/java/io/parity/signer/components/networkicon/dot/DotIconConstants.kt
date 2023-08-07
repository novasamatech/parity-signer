package io.parity.signer.components.networkicon.dot

import io.parity.signer.components.networkicon.dot.DotIconColors.SchemeElement


internal object DotIconConstants {
	/// Function to set default coloring schemes, taken as is from js code
	fun defaultSchemes() = listOf(
			SchemeElement(
				// "target"
				freq =  1u,
				colors = listOf(0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 0, 28, 0, 1)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "cube",
				freq =  20u,
				colors = listOf(0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 0, 1, 3, 2, 4, 3, 5)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "quazar",
				freq =  16u,
				colors = listOf(1, 2, 3, 1, 2, 4, 5, 5, 4, 1, 2, 3, 1, 2, 4, 5, 5, 4, 0)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "flower",
				freq =  32u,
				colors = listOf(0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 3)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "cyclic",
				freq =  32u,
				colors = listOf(0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 6)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "vmirror",
				freq = 128u,
				colors = listOf(0, 1, 2, 3, 4, 5, 3, 4, 2, 0, 1, 6, 7, 8, 9, 7, 8, 6, 10)
					.map { it.toUByte() },
			),
			SchemeElement(
				// "hmirror",
				freq =  128u,
				colors = listOf(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 6, 7, 5, 3, 4, 2, 11)
					.map { it.toUByte() },
			),
		)

	internal val previewAliceSeed = listOf(212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125).map { it.toUByte() }

	internal val previewBobSeed = listOf(142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147,
		201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,).map { it.toUByte() }

}
