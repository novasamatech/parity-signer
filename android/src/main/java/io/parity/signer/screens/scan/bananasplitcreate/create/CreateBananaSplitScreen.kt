package io.parity.signer.screens.scan.bananasplitcreate.create

import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit


@Composable
fun CreateBananaSplitScreen(coreNavController: NavController) {
	val vm: CreateBsViewModel = viewModel()

	var passPhrase = rememberSaveable() {
		vm.generatePassPhrase(BananaSplit.defaultShards)
			.handleErrorAppState(coreNavController) ?: ""
	}

	CreateBananaSplitScreenInternal(
		onClose = { coreNavController.popBackStack() },
		onCreate = { shards ->
			vm.createBS(shards, passPhrase)
			//todo dmitry navigate to next screen
		},
		updatePassowrd = {
			passPhrase = vm.generatePassPhrase(BananaSplit.defaultShards)
				.handleErrorAppState(coreNavController) ?: ""
		},
		password = passPhrase,
		modifier = Modifier.statusBarsPadding(),
	)
}
