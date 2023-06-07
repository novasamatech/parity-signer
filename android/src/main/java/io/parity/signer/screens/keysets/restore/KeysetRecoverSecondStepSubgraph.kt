package io.parity.signer.screens.keysets.restore

import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.restore.restorephrase.KeysetRecoverPhraseScreen


@Composable
fun NewKeysetRecoverSecondStepSubgraph(
	rootNavigator: Navigator,
	initialRecoverSeedPhrase: KeysetRecoverModel,
) {
//background
	Box(
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	val navController = rememberNavController()
	NavHost(
		navController = navController,
		startDestination = KeysetRecoverSubgraph.KeysetRecoverSeed,
	) {
		composable(KeysetRecoverSubgraph.KeysetRecoverSeed) {
			val context = LocalContext.current
			//todo dmitry viewmodel? add
			KeysetRecoverPhraseScreen(
				onContinue = { seed ->
					viewModel.resetState()
					viewModel.addSeed(
						seedName = state.seedName,
						seedPhrase = seedFinal,
						navigator = rootNavigator
					)
					Toast.makeText(
						context,
						context.getText(R.string.key_set_has_been_recovered_toast),
						Toast.LENGTH_LONG
					).show()
										 },//todo dmitry
				onBack = {rootNavigator.backAction()},
				rootNavigator = rootNavigator,
				initialRecoverSeedPhrase = initialRecoverSeedPhrase,
			)
			BackHandler(onBack = rootNavigator::backAction)
		}
		composable(KeysetRecoverSubgraph.KeysetRecoverNetworks) {
			RecoverKeysetSelectNetworkScreen(
				seedName = initialRecoverSeedPhrase.seedName,
				seedPhrase = initialRecoverSeedPhrase.readySeed!!,//todo dmitry this is not last
				rootNavigator = rootNavigator,
				onBack = navController::popBackStack,
				modifier = Modifier.statusBarsPadding(),
			)
		}
	}
}

internal object KeysetRecoverSubgraph {
	const val KeysetRecoverSeed = "new_keyset_backup_main"
	const val KeysetRecoverNetworks = "new_keyset_backup_confirmation"
}
