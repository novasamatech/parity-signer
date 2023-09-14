package io.parity.signer.screens.settings.networks.signnetworkcrypto

import androidx.compose.runtime.Composable
import io.parity.signer.screens.settings.networks.signnetworkcrypto.view.SignSufficientCryptoScreen
import io.parity.signer.uniffi.MSignSufficientCrypto
import io.parity.signer.uniffi.ScreenData


@Composable
fun SignSufficientCryptoFull(sc: MSignSufficientCrypto) {

	SignSufficientCryptoScreen(sc,
		signSufficientCrypto = { seedName: String, addressKey: String ->
//			sharedViewModel::signSufficientCrypto
			Unit
		}
	)
}
