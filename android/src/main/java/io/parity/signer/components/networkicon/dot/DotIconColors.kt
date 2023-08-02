package io.parity.signer.components.networkicon.dot

import com.appmattus.crypto.Algorithm


internal object DotIconColors {

	private object Constants {
		val byteHashLength = 64
		val arrayZeroBytesLength = 32
		val derivedIDRotationFactorMultiplier: Int = 6
		val derivedIDRotationFactorModulo: Int = 3
		val hueDegrees = 360
		val colorArrayLength = 19
		val lightnessPercentages = listOf(53, 15, 35, 75)
	}

	/**
	 * Function to calculate identicon colors from `&[u8]` input slice.
	 * Total 19 colors are always produced.
	 *
	 * As colors.rs:140 in polkadot-identicon-rust
	 */
	@OptIn(ExperimentalUnsignedTypes::class)
	fun getColors(seed: String): List<DotIconColor> {
		val seedInBytes = seed.toByteArray()
		val black2b = Algorithm.Blake2b(64).createDigest()

		val zeros: ByteArray = black2b.digest(ByteArray(32) { 0u.toByte() })
		val idPrep: ByteArray = black2b.digest(seedInBytes)

		val id: UByteArray = idPrep
			.mapIndexed { index, byte -> (byte.toUByte() - zeros[index].toUByte()).toUByte() }
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
					0u.toUByte() -> DotIconColor(
						red = 4u,
						green = 4u,
						blue = 4u,
						alpha = 255u,
					)
					255u.toUByte() -> DotIconColor(
						red = 4u,
						green = 4u,
						blue = 4u,
						alpha = 255u,
					)
					else -> DotIconColor.derive(b, sat_component)
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
		val myColors: List<DotIconColor> = List(19) { i ->
			val numColor = if (i < 19) (i + rot) % 18 else 18
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

	internal data class DotIconColor(
		val red: UByte,
		val green: UByte,
		val blue: UByte,
		val alpha: UByte,
	) {
		companion object {
			val background
				get() = DotIconColor(
					red = 255u,
					green = 255u,
					blue = 255u,
					alpha = 0u,
				)

			val foreground
				get() = DotIconColor(
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
			fun derive(b: UByte, set_component: Double): DotIconColor {
				//todo dmitry as colors.rs:99
				return DotIconColor.foreground //todo dmitry remove
			}
		}
	}
}
