package io.parity.signer.screens.scan.bananasplitcreate.create

import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
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

	val passPhrase: MutableState<String> = rememberSaveable() {
		mutableStateOf(
			vm.generatePassPhrase(BananaSplit.defaultShards)
				.handleErrorAppState(coreNavController) ?: ""
		)
	}

	CreateBananaSplitScreenInternal(
		onClose = { coreNavController.popBackStack() },
		onCreate = { maxShards ->
			vm.viewModelScope.launch {
				val result = vm.createBS(seedName, maxShards, passPhrase.value)
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
			passPhrase.value = vm.generatePassPhrase(BananaSplit.defaultShards)
				.handleErrorAppState(coreNavController) ?: ""
		},
		password = passPhrase.value,
		modifier = Modifier.statusBarsPadding(),
	)
}
