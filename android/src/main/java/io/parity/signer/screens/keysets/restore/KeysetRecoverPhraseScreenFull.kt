package io.parity.signer.screens.keysets.restore

import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreenView
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun KeysetRecoverPhraseScreenFull(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
	var keySetName by remember { mutableStateOf("") }

	val canProceed = keySetName.isNotEmpty() //&& !seedNames.contains(keySetName)
	val viewModel = viewModel<KeysetRecoverViewModel>()

	KeysetRecoverPhraseScreenView(
		recoverSeedPhrase = initialRecoverSeedPhrase,
		addSeed = { seedName,
								seedPhrase ->
			viewModel.addSeed(
				seedName = seedName,
				seedPhrase = seedPhrase,
				navigator = rootNavigator
			)
		})
}




