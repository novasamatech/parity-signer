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
	fun calculatePositionsCircleSet(full: Dp): List<DotIconCirclePosition> {
		val centerToCenter = full / 8 * 3
		val a = centerToCenter;
		val b = centerToCenter * sqrt(3f) / 2f;

		return listOf(
			DotIconCirclePosition(x = 0.dp, y = a * -2),
			DotIconCirclePosition(x = 0.dp, y = -a),
			DotIconCirclePosition(x = -b, y = a * -3 / 2),
			DotIconCirclePosition(x = b * -2, y = -a),
			DotIconCirclePosition(x = -b, y = -a / 2),
			DotIconCirclePosition(x = b * -2, y = 0.dp),
			DotIconCirclePosition(x = b * -2, y = a),
			DotIconCirclePosition(x = -b, y = a / 2),
			DotIconCirclePosition(x = -b, y = a * 3 / 2),
			DotIconCirclePosition(x = 0, y = a * 2),
			DotIconCirclePosition(x = 0, y = a),
			DotIconCirclePosition(x = b, y = a * 3 / 2),
			DotIconCirclePosition(x = b * 2, y = a),
			DotIconCirclePosition(x = b, y = a / 2),
			DotIconCirclePosition(x = b * 2, y = 0),
			DotIconCirclePosition(x = b * 2, y = -a),
			DotIconCirclePosition(x = b, y = -a / 2),
			DotIconCirclePosition(x = b, y = a * -3 / 2),
			DotIconCirclePosition(x = 0.dp, y = 0.dp),
		)
	}
}

/**
 * Paddings from center
 */
internal data class DotIconCirclePosition(val x: Dp, val y: Dp)
