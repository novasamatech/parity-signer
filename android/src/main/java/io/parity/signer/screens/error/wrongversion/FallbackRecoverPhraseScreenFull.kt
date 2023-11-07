package io.parity.signer.screens.error.wrongversion

import androidx.compose.runtime.Composable
import io.parity.signer.domain.Callback

@Composable
fun FallbackRecoverPhraseScreenFull(onBack: Callback) {
	FallbackRecoverPhraseScreen(seedList = listOf(""),//todo dmitry
		onSelected = {},//todo dmitry
		onBack = onBack,
	)
}
