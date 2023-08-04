package io.parity.signer.components.networkicon.dot

import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import kotlin.math.sqrt


internal object DotIconPositions {


	/**
	 *
	 *  Set default positions of small circles in 19-circles icon
	 *  Calculates the positions for the centers of the circles in a radial layout.
	 *
	 *  The layout is as follows:
	 *
	 *                              0
	 *
	 *                     2               17
	 *
	 *            3                 1              15
	 *
	 *                     4               16
	 *
	 *            5                18              14
	 *
	 *                     7               13
	 *
	 *            6                10              12
	 *
	 *                     8               11
	 *
	 *                              9
	 */
	fun calculatePositionsCircleSet(fullSize: Dp): List<DotIconCircleOffset> {
		val bigCircleRadius = fullSize / 2
		val centerToCenter = bigCircleRadius / 8 * 3
		val a = centerToCenter;
		val b = centerToCenter * sqrt(3f) / 2f;

		return listOf(
			DotIconCircleOffset(x = 0.dp, y = a * -2),
			DotIconCircleOffset(x = 0.dp, y = -a),
			DotIconCircleOffset(x = -b, y = a * -3 / 2),
			DotIconCircleOffset(x = b * -2, y = -a),
			DotIconCircleOffset(x = -b, y = -a / 2),
			DotIconCircleOffset(x = b * -2, y = 0.dp),
			DotIconCircleOffset(x = b * -2, y = a),
			DotIconCircleOffset(x = -b, y = a / 2),
			DotIconCircleOffset(x = -b, y = a * 3 / 2),
			DotIconCircleOffset(x = 0.dp, y = a * 2),
			DotIconCircleOffset(x = 0.dp, y = a),
			DotIconCircleOffset(x = b, y = a * 3 / 2),
			DotIconCircleOffset(x = b * 2, y = a),
			DotIconCircleOffset(x = b, y = a / 2),
			DotIconCircleOffset(x = b * 2, y = 0.dp),
			DotIconCircleOffset(x = b * 2, y = -a),
			DotIconCircleOffset(x = b, y = -a / 2),
			DotIconCircleOffset(x = b, y = a * -3 / 2),
			DotIconCircleOffset(x = 0.dp, y = 0.dp),
		)
	}
}

/**
 * offsets from center
 */
internal data class DotIconCircleOffset(val x: Dp, val y: Dp)
