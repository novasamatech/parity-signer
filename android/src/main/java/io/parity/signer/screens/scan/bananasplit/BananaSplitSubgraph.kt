package io.parity.signer.screens.scan.bananasplit

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.bananasplit.networks.RecoverKeysetSelectNetworkBananaFlowScreen
import kotlinx.coroutines.flow.filterNotNull
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking


@Composable
fun BananaSplitSubgraph(
	qrData: List<String>,
	onClose: Callback,
	onSuccess: (newSeed: String) -> Unit,
	onErrorWrongPassword: Callback,
	onCustomError: (errorText: String) -> Unit,
) {

	val bananaViewModel: BananaSplitViewModel = viewModel()

	LaunchedEffect(Unit) {
		bananaViewModel.initState(qrData)

		launch {
			bananaViewModel.isWrongPasswordTerminal.collect {
				if (it) {
					onErrorWrongPassword()
					bananaViewModel.cleanState()
				}
			}
		}
		launch {
			bananaViewModel.isCustomErrorTerminal
				.filterNotNull()
				.collect {
					onCustomError(it)
					bananaViewModel.cleanState()
				}
		}
		launch {
			bananaViewModel.isSuccessTerminal
				.filterNotNull()
				.collect {
					onSuccess(it)
					bananaViewModel.cleanState()
				}
		}
	}

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
		startDestination = BananaSplitNavigationSubgraph.BananaSplitNavigationPassword,
	) {
		composable(BananaSplitNavigationSubgraph.BananaSplitNavigationPassword) {
			BananaSplitPasswordScreen(
				onClose = onClose,
				onDone = {
					navController.navigate(BananaSplitNavigationSubgraph.BananaSplitNavigationNetworks)
				},
				bananaViewModel = bananaViewModel,
				modifier = Modifier.statusBarsPadding(),
			)
			BackHandler(onBack = onClose)
		}
		composable(BananaSplitNavigationSubgraph.BananaSplitNavigationNetworks) {
			val context = LocalContext.current
			RecoverKeysetSelectNetworkBananaFlowScreen(
				onBack = navController::popBackStack,
				onDone = { networks ->
					runBlocking { bananaViewModel.onDoneTap(context, networks) }
				}
			)
		}
	}
}


private object BananaSplitNavigationSubgraph {
	const val BananaSplitNavigationPassword = "banana_split_step_password"
	const val BananaSplitNavigationNetworks = "banana_split_step_networks"
}
