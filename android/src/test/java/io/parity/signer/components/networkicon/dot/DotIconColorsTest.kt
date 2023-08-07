package io.parity.signer.components.networkicon.dot

import org.junit.Assert
import org.junit.Test


class DotIconColorsTest {

	@Test
	fun checkColor2() {
		val seed = DotIconConstants.previewAliceSeed
		val colors = DotIconColors.getColors(seed)
		Assert.assertEquals("184", colors[2].red.toString())
	}

	@Test
	fun checkColorLast() {
		val seed = DotIconConstants.previewAliceSeed
		val colors = DotIconColors.getColors(seed)
		Assert.assertEquals("61", colors.last().red.toString())
	}

	@Test
	fun checkColorDerivation() {
		val b = 212u.toUByte()
		val sat = 0.56
		val color = DotIconColors.DotIconColorRgb.derive(b, sat)
		Assert.assertEquals(DotIconColors.DotIconColorRgb(165u, 227u, 156u, 255u), color)
	}

	@Test
	fun checkColorsFull() {
		val seed = DotIconConstants.previewAliceSeed
		val colors = DotIconColors.getColors(seed)
		Assert.assertEquals(aliceColor, colors)
	}

	//taken from rust samples
	private val aliceColor
		get() = listOf(
			DotIconColors.DotIconColorRgb(
				red = 165u,
				green = 227u,
				blue = 156u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 60u,
				green = 40u,
				blue = 17u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 184u,
				green = 68u,
				blue = 202u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 139u,
				green = 39u,
				blue = 88u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 135u,
				green = 68u,
				blue = 202u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 225u,
				green = 156u,
				blue = 227u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 139u,
				green = 39u,
				blue = 88u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 135u,
				green = 68u,
				blue = 202u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 184u,
				green = 68u,
				blue = 202u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 165u,
				green = 227u,
				blue = 156u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 60u,
				green = 40u,
				blue = 17u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 162u,
				green = 202u,
				blue = 68u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 39u,
				green = 139u,
				blue = 139u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 187u,
				green = 202u,
				blue = 68u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 38u,
				green = 60u,
				blue = 17u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 39u,
				green = 139u,
				blue = 139u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 187u,
				green = 202u,
				blue = 68u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 162u,
				green = 202u,
				blue = 68u,
				alpha = 255u
			),
			DotIconColors.DotIconColorRgb(
				red = 61u,
				green = 39u,
				blue = 139u,
				alpha = 255u
			),
		)
}


