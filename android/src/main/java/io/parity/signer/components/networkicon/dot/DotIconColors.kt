package io.parity.signer.components.networkicon.dot

import androidx.compose.material.Colors
import com.appmattus.crypto.Algorithm


internal object DotIconColors {

	/**
	 * Function to calculate identicon colors from `&[u8]` input slice.
	 * Total 19 colors are always produced.
	 *
	 * As colors.rs:140 in polkadot-identicon-rust
	 */

	fun getColors(seed: String): List<Colors> {

		val black2b = Algorithm.Blake2b(64).createDigest()
		return emptyList()
	}
}
