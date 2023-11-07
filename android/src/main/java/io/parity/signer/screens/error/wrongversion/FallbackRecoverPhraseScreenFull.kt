package io.parity.signer.screens.error.wrongversion

import androidx.compose.runtime.Composable
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Callback

@Composable
fun FallbackRecoverPhraseScreenFull(onBack: Callback) {

	val viewModel: FallbackRecoverPhraseViewModel =  viewModel()
	FallbackRecoverPhraseScreen(seedList = viewModel.getSeedsList(),
		onSelected = {},//todo dmitry
		onBack = onBack,
	)


}
