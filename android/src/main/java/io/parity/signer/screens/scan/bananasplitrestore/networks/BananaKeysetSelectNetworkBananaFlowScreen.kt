package io.parity.signer.screens.scan.bananasplitrestore.networks

import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.domain.Callback
import io.parity.signer.screens.keysets.restore.recoverkeysetnetworks.RecoverKeysetSelectNetworkScreenBase


@Composable
fun RecoverKeysetSelectNetworkBananaFlowScreen(
	onBack: Callback,
	onDone: (networksKeys: Set<String>) -> Unit,
) {
	val networksViewModel: BananaNetworksViewModel = viewModel()
	val defaultSelectedNetworks =
		networksViewModel.getDefaultPreselectedNetworks()
			.map { it.key }
			.toSet()
	val selected: MutableState<Set<String>> =
		remember {
			mutableStateOf(
				defaultSelectedNetworks
			)
		}
	val networks = networksViewModel.getAllNetworks()

	RecoverKeysetSelectNetworkScreenBase(
		onProceedAction = { onDone(selected.value) },
		networks = networks,
		selected = selected,
		defaultSelectedNetworks = defaultSelectedNetworks,
		onBack = onBack
	)
}
