package io.parity.signer.components.networkicon.dot

import androidx.compose.ui.graphics.Color
import com.appmattus.crypto.Algorithm


internal object DotIconColors {

	/**
	 * Function to calculate identicon colors from `&[u8]` input slice.
	 * Total 19 colors are always produced.
	 *
	 * As colors.rs:140 in polkadot-identicon-rust
	 */
	@OptIn(ExperimentalUnsignedTypes::class)
	fun getColors(seed: List<UByte>): List<DotIconColorRgb> {
		val seedInBytes = seed.toUByteArray().toByteArray()
		val byte = 8
		val black2b = Algorithm.Blake2b(64 * byte).createDigest()

		val zeros: UByteArray = black2b.digest(ByteArray(32) { 0u.toByte() }).toUByteArray()
		val idPrep: UByteArray = black2b.digest(seedInBytes).toUByteArray()

		val id: UByteArray = idPrep
			.mapIndexed { index, bytes -> (bytes - zeros[index]).toUByte() }
			.toUByteArray()

		// this comment from Rust code
		// Since `id[29]` is u8, `sat` could range from 30 to 109, i.e. it always fits into u8.
		// Transformation of id[29] into u16 is to avoid overflow in multiplication
		// (wrapping could be used, but is more bulky).
		// (this is taken as is from js code).
		// However, this way `sat_component` could have values above 1.00.
		// Palette crate does not check at this moment that `sat_component` is not
		// overflowing 1.00, and produces some kind of resulting color.
		// Need to find out what should have happened if the sat values are above 100.

		val sat = (((id[29].toUShort() * 70u / 256u + 26u) % 80u) + 30u).toUByte();
		val sat_component: Double = (sat.toDouble()) / 100;

		// calculating palette: set of 32 RGBA colors to be used in drawing
		// only id vector is used for this calculation

		val myPalette = id.mapIndexed { index: Int, byte: UByte ->
			val newColor =
				when (val b = (byte + ((index.toUByte() % 28u) * 58u)).toUByte()) {
					0u.toUByte() -> DotIconColorRgb(
						red = 4u,
						green = 4u,
						blue = 4u,
						alpha = 255u,
					)

					255u.toUByte() -> DotIconColorRgb.foreground

					else -> DotIconColorRgb.derive(b, sat_component)
				}
			newColor
		}

		// loading default coloring schemes
		val schemes = DotIconConstants.defaultSchemes()

		// `total` is the sum of frequencies for all scheme elements in coloring schemes,
		// in current setting is always 357
		val total = schemes.sumOf { it.freq.toInt() }

		// `d` is used to determine the coloring scheme to be used.
		// Transformation into u32 is used to avoid overflow.
		val d = (id[30].toInt() + (id[31].toInt()) * 256) % total;

		// determining the coloring scheme to be used
		val myScheme = chooseScheme(schemes, d)

		// calculating rotation for the coloring scheme
		val rot: Byte = (id[28] % 6u * 3u).toByte()

		// picking colors from palette using coloring scheme with rotation applied
		val myColors: List<DotIconColorRgb> = List(19) { i ->
			val numColor = if (i < 18) (i + rot) % 18 else 18
			val numPalette = myScheme.colors[numColor].toInt()
			val color = myPalette[numPalette]
			color
		}

		return myColors
	}

	/**
	 * Function to choose the coloring scheme based on value d.
	 * Note that d is calculated as remainder of division by total sum of frequencies,
	 * so it can not exceed the total sum of frequencies
	 */
	private fun chooseScheme(
		schemes: List<SchemeElement>,
		d: Int
	): SchemeElement {
		var sum = 0
		for (x in schemes) {
			sum += x.freq.toInt()
			if (d < sum) {
				return x
			}
		}
		throw IllegalStateException("should always be determined: d is calculated as remainder of division by total sum of frequencies, so it can not exceed the total sum of frequencies")
	}


	/// Struct to store default coloring schemes
	internal data class SchemeElement(val freq: UByte, val colors: List<UByte>)

	/**
	 * ARGB representation
	 */
	internal data class DotIconColorRgb(
		val red: UByte,
		val green: UByte,
		val blue: UByte,
		val alpha: UByte,
	) {
		fun toCompose(): Color = Color(
			this.red.toInt(),
			this.green.toInt(),
			this.blue.toInt(),
			this.alpha.toInt(),
		)

		companion object {
			val background
				get() = DotIconColorRgb(
					red = 255u,
					green = 255u,
					blue = 255u,
					alpha = 0u,
				)

			val foreground
				get() = DotIconColorRgb(
					red = 238u,
					green = 238u,
					blue = 238u,
					alpha = 255u,
				)

			/**
			 * function to derive color from `u8` number and saturation component
			 * calculated elsewhere;
			 * is accessible and used only for `u8` numbers other than 0 and 255;
			 * no check here is done for b value;
			 */
			fun derive(b: UByte, saturation: Double): DotIconColorRgb {
				// HSL color hue in degrees
				val hueModulus = 64u
				val hue: Float = ((b.toUShort() % hueModulus * 360u) / hueModulus).toFloat()

				// HSL lightness in percents
				val l: UByte = when (b / 64u) {
					0u -> 53u
					1u -> 15u
					2u -> 35u
					else -> 75u
				}
				// recalculated in HSL lightness component (component range is 0.00 to 1.00)
				val l_component = (l.toDouble()) / 100;
				return hslToRgb(hue, saturation.toFloat(), l_component.toFloat())
			}

			/**
			 * Converts HSL color space values to RGB color space values.
			 *
			 * Implementation of /androidx/core/graphics/ColorUtils.java:318 - HSLToColor()
			 *
			 * @param hue: The hue value of the HSL color, specified as a degree between 0 and 360.
			 * @param saturation: The saturation value of the HSL color, specified as a double between 0 and 1.
			 * @param lightness: The lightness value of the HSL color, specified as a double between 0 and 1.
			 * @return Returns: A tuple representing the RGB color values, each a UInt8 between 0 and 255.
			 */
			private fun hslToRgb(
				hue: Float,
				saturation: Float,
				lightness: Float
			): DotIconColorRgb {

				val c = (1f - Math.abs(2 * lightness - 1f)) * saturation
				val m = lightness - 0.5f * c
				val x = c * (1f - Math.abs(hue / 60f % 2f - 1f))

				val hueSegment = hue.toInt() / 60

				var r = 0
				var g = 0
				var b = 0

				when (hueSegment) {
					0 -> {
						r = Math.round(255 * (c + m))
						g = Math.round(255 * (x + m))
						b = Math.round(255 * m)
					}
					1 -> {
						r = Math.round(255 * (x + m))
						g = Math.round(255 * (c + m))
						b = Math.round(255 * m)
					}
					2 -> {
						r = Math.round(255 * m)
						g = Math.round(255 * (c + m))
						b = Math.round(255 * (x + m))
					}
					3 -> {
						r = Math.round(255 * m)
						g = Math.round(255 * (x + m))
						b = Math.round(255 * (c + m))
					}
					4 -> {
						r = Math.round(255 * (x + m))
						g = Math.round(255 * m)
						b = Math.round(255 * (c + m))
					}
					5, 6 -> {
						r = Math.round(255 * (c + m))
						g = Math.round(255 * m)
						b = Math.round(255 * (x + m))
					}
				}
				return DotIconColorRgb(
					red = r.toUByte(),
					green = g.toUByte(),
					blue = b.toUByte(),
					alpha = 255u
				)
			}


			/**
			 * Calculates a single RGB color component from HSL values.
			 *
			 * @param p: The first helper value derived from the lightness value of the HSL color.
			 * @param q: The second helper value derived from the lightness and saturation values of the HSL color.
			 * @param hueShift: The hue value of the HSL color, shifted by a certain amount.
			 * @return  A double representing the calculated RGB color component.
			 */
			private fun convertHueToRgbComponent(
				p: Float,
				q: Float,
				hueShift: Float
			): Float {
				var shiftedHue = hueShift

				if (shiftedHue < 0f) {
					shiftedHue += 1f
				}
				if (shiftedHue > 1f) {
					shiftedHue -= 1f
				}
				return when {
					shiftedHue < (1 / 6f) -> p + (q - p) * 6 * shiftedHue
					shiftedHue < (1 / 2f) -> q
					shiftedHue < (2 / 3f) -> p + (q - p) * (2 / 3 - shiftedHue) * 6
					else -> p
				}
			}
		}
	}
}
