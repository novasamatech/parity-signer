package io.parity.signer.screens.keysets.restore.restorephrase

import android.widget.Toast
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.keysets.create.backupstepscreens.NewKeySetNetworksWithNavigatorViewModel
import io.parity.signer.screens.keysets.restore.recoverkeysetnetworks.RecoverKeysetSelectNetworkScreenBase


@Composable
fun RecoverKeysetSelectNetworkRestoreFlowScreen(
	seedName: String,
	seedPhrase: String,
	rootNavigator: Navigator,
	onBack: Callback,
) {
	val networksViewModel: NewKeySetNetworksWithNavigatorViewModel = viewModel()
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

	val context = LocalContext.current
	val onProceedAction = {
		networksViewModel.createKeySetWithNetworks(
			seedName = seedName, seedPhrase = seedPhrase,
			networksForKeys = selected.value.mapNotNull { selected -> networks.find { it.key == selected } }
				.toSet(),
			navigator = rootNavigator,
			onPostReaction = { isSuccess ->
				if (isSuccess) {
					Toast.makeText(
						context,
						context.getText(R.string.key_set_has_been_recovered_toast),
						Toast.LENGTH_LONG
					).show()
				}
			}
		)
	}

	RecoverKeysetSelectNetworkScreenBase(
		onProceedAction = onProceedAction,
		networks = networks,
		selected = selected,
		defaultSelectedNetworks = defaultSelectedNetworks,
		onBack = onBack
	)
}
