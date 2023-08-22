package io.parity.signer.screens.scan.bananasplit

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.bananasplit.networks.RecoverKeysetSelectNetworkBananaFlowScreen
import kotlinx.coroutines.flow.filterNotNull
import kotlinx.coroutines.launch


@Composable
fun BananaSplitSubgraph(
	qrData: List<String>,
	onClose: Callback,
	onSuccess: (newSeed: String) -> Unit,
	onErrorWrongPassword: Callback,
	onCustomError: (errorText: String) -> Unit,
) {

	val bananaViewModel: BananaSplitViewModel = viewModel()

	DisposableEffect(qrData) {
		bananaViewModel.initState(qrData)
		onDispose {
			bananaViewModel.cleanState()
		}
	}
	LaunchedEffect(key1 = qrData) {
		launch {
			bananaViewModel.isWrongPasswordTerminal.collect {
				if (it) {
					onErrorWrongPassword()
				}
			}
		}
		launch {
			bananaViewModel.isCustomErrorTerminal
				.filterNotNull()
				.collect {
					onCustomError(it)
				}
		}
		launch {
			bananaViewModel.isSuccessTerminal
				.filterNotNull()
				.collect {
					onSuccess(it)
				}
		}
	}

	val isBananaRestorable = bananaViewModel.isBananaRestorable.collectAsState()
	val context = LocalContext.current

	BackHandler(onBack = {
		if (isBananaRestorable.value) {
			bananaViewModel.backToBananaRestore()
		} else {
			onClose()
		}
	})
	//background
	Box(
		modifier = Modifier
			.fillMaxSize(1f)
			.statusBarsPadding()
			.background(MaterialTheme.colors.background)
	)

	if (!isBananaRestorable.value) {
		BananaSplitPasswordScreen(
			onClose = onClose,
			onDone = {
				bananaViewModel.viewModelScope.launch {
					bananaViewModel.onBananaDoneTry(context)
				}
			},
			bananaViewModel = bananaViewModel,
			modifier = Modifier.statusBarsPadding(),
		)
	} else {
		RecoverKeysetSelectNetworkBananaFlowScreen(
			onBack = bananaViewModel::backToBananaRestore,
			onDone = { networks ->
				bananaViewModel.viewModelScope.launch {
					bananaViewModel.onFinishWithNetworks(context, networks)
				}
			}
		)
	}
}


