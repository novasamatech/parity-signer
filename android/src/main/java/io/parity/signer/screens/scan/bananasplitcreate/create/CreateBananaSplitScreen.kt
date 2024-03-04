package io.parity.signer.screens.scan.bananasplitcreate.create

import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import io.parity.signer.screens.error.handleErrorAppState
import io.parity.signer.screens.scan.bananasplitcreate.BananaSplit
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import kotlinx.coroutines.launch


@Composable
fun CreateBananaSplitScreen(
	coreNavController: NavController,
	seedName: String,
) {
	val vm: CreateBsViewModel = viewModel()

	var passPhrase = rememberSaveable() {
		vm.generatePassPhrase(BananaSplit.defaultShards)
			.handleErrorAppState(coreNavController) ?: ""
	}

	CreateBananaSplitScreenInternal(
		onClose = { coreNavController.popBackStack() },
		onCreate = { maxShards ->
			vm.viewModelScope.launch {
				val result = vm.createBS(seedName, maxShards, passPhrase)
				result.handleErrorAppState(coreNavController)?.let {
					coreNavController.popBackStack()
					coreNavController.navigate(
						CoreUnlockedNavSubgraph.CreateBananaSplit.destination(
							seedName
						)
					)
				}
			}
		},
		updatePassowrd = {
			passPhrase = vm.generatePassPhrase(BananaSplit.defaultShards)
				.handleErrorAppState(coreNavController) ?: ""
		},
		password = passPhrase,
		modifier = Modifier.statusBarsPadding(),
	)
}