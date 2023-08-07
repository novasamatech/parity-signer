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
	fun checkColorsAlice() {
		val seed = DotIconConstants.previewAliceSeed
		val colors = DotIconColors.getColors(seed)
		Assert.assertEquals(aliceColor, colors)
	}

	@Test
	fun checkColorsBob() {
		val seed = DotIconConstants.previewBobSeed
		val colors = DotIconColors.getColors(seed)
		Assert.assertEquals(bobColor, colors)
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

	private val bobColor
		get() = listOf(
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 61u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 200u, green = 214u, blue = 169u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 214u, green = 169u, blue = 182u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 36u, green = 52u, blue = 25u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 127u, green = 93u, blue = 177u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 214u, green = 169u, blue = 182u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 61u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 200u, green = 214u, blue = 169u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 52u, green = 25u, blue = 30u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 113u, green = 177u, blue = 93u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 114u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 108u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 118u, green = 93u, blue = 177u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 25u, green = 52u, blue = 39u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 108u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 113u, green = 177u, blue = 93u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 58u, green = 120u, blue = 114u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 52u, green = 25u, blue = 30u, alpha = 255u),
			DotIconColors.DotIconColorRgb(red = 33u, green = 25u, blue = 52u, alpha = 255u),
		)
}


