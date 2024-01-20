package io.parity.signer.screens.keysets.create.backupstepscreens

import android.content.res.Configuration
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.MaterialTheme
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.Text
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.bottomsheets.ProceedEmptyKeysetConfirmation
import io.parity.signer.components.base.NotificationFrameText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeaderProgressWithButton
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.items.NetworkItemMultiselect
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.ui.BottomSheetWrapperContent
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import kotlinx.coroutines.launch


@OptIn(ExperimentalMaterialApi::class)
@Composable
fun NewKeySetSelectNetworkScreen(
	seedName: String,
	seedPhrase: String,
	onBack: Callback,
	showError: (AuthOperationResult) -> Unit,
	onSuccess: Callback,
) {
	val networksViewModel: NewKeySetNetworksViewModel = viewModel()
	val selected: MutableState<Set<String>> =
		remember {
			mutableStateOf(
				networksViewModel.getDefaultPreselectedNetworks()
					.map { it.key }.toSet()
			)
		}
	val networks = networksViewModel.getAllNetworks()

	val context = LocalContext.current
	val confirmBottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmValueChange = {
				it != ModalBottomSheetValue.HalfExpanded
			},
			skipHalfExpanded = false
		)
	val scope = rememberCoroutineScope()

	val onProceedAction = {
		networksViewModel.createKeySetWithNetworks(
			seedName = seedName, seedPhrase = seedPhrase,
			networkForKeys = selected.value.mapNotNull { selected -> networks.find { it.key == selected } }
				.toSet(),
			onAfterCreate = { success ->
				when (success) {
					is AuthOperationResult.AuthFailed,
					is AuthOperationResult.Error -> {
						showError(success)
					}

					AuthOperationResult.Success -> {
						Toast.makeText(
							context,
							context.getString(
								R.string.key_set_has_been_created_toast,
								seedName
							),
							Toast.LENGTH_LONG
						).show()
						onSuccess()
					}
				}
			}
		)
	}

	BottomSheetWrapperContent(
		bottomSheetState = confirmBottomSheetState,
		bottomSheetContent = {
			ProceedEmptyKeysetConfirmation(
				onCancel = { scope.launch { confirmBottomSheetState.hide() } },
				onProceed = onProceedAction,
			)
		},
		mainContent = {
			NewKeySetSelectNetworkScreenPrivate(
				networks = networks,
				selectedNetworkKeys = selected.value,
				onNetworkClick = { network ->
					selected.value = if (selected.value.contains(network.key)) {
						selected.value - network.key
					} else {
						selected.value + network.key
					}
				},
				onProceed = {
					if (selected.value.isNotEmpty()) {
						onProceedAction()
					} else {
						scope.launch { confirmBottomSheetState.show() }
					}
				},
				onAddAll = {
					selected.value = if (selected.value.size == networks.size) {
						networksViewModel.getDefaultPreselectedNetworks().map { it.key }
							.toSet()
					} else {
						networks.map { it.key }.toSet()
					}
				},
				onBack = onBack,
			)
		},
	)
}

@Composable
private fun NewKeySetSelectNetworkScreenPrivate(
	networks: List<NetworkModel>,
	selectedNetworkKeys: Set<String>,
	onNetworkClick: (NetworkModel) -> Unit,
	onProceed: Callback,
	onAddAll: Callback,
	onBack: Callback
) {
	Column(
		modifier = Modifier
			.fillMaxSize(1f)
			.background(MaterialTheme.colors.background)
			.verticalScroll(rememberScrollState()),
	) {
		ScreenHeaderProgressWithButton(
			canProceed = false,
			currentStep = 3,
			allSteps = 3,
			btnText = stringResource(R.string.button_next),
			onClose = onBack,
			onButton = null,
			backNotClose = true,
		)
		Text(
			text = stringResource(R.string.keyset_create_keys_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(bottom = 8.dp),
		)
		Text(
			text = stringResource(R.string.keyset_create_keys_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
		)
		Column(
			modifier = Modifier
				.padding(horizontal = 8.dp, vertical = 16.dp)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
				)
		) {
			networks.forEach { network ->
				NetworkItemMultiselect(
					network = network,
					isSelected = selectedNetworkKeys.contains(network.key)
				) { network ->
					onNetworkClick(network)
				}
				SignerDivider()
			}
			NetworkItemMultiselectAll(onAddAll)
		}
		NotificationFrameText(
			message = stringResource(R.string.keyset_create_keys_notification_text),
			modifier = Modifier
				.padding(horizontal = 16.dp)
		)
		Spacer(modifier = Modifier.weight(1f))

		PrimaryButtonWide(
			label = stringResource(R.string.generic_done),
			modifier = Modifier.padding(horizontal = 32.dp, vertical = 24.dp),
			onClicked = onProceed,
		)
	}
}


@Composable
internal fun NetworkItemMultiselectAll(
	onClick: Callback,
) {
	Row(
		modifier = Modifier
			.clickable(onClick = onClick)
			.height(68.dp)
			.padding(horizontal = 16.dp)
			.fillMaxWidth(1f),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = stringResource(R.string.keyset_create_keys_select_all),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
		)
	}
}


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewNewKeySetSelectNetwork() {
	val networks = listOf(
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
	)
	val selected = setOf(networks[1].key)
	SignerNewTheme {
		NewKeySetSelectNetworkScreenPrivate(networks, selected, {}, {}, {}, {})
	}
}

@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewNewKeySetSelectNetworkVeryLong() {
	val networks = listOf(
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "0",
			logo = "polkadot",
			title = "Polkadot",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "1",
			logo = "Kusama",
			title = "Kusama",
			pathId = "polkadot",
		),
		NetworkModel(
			key = "2",
			logo = "Wastend",
			title = "Wastend",
			pathId = "polkadot",
		),
	)
	val selected = setOf(networks[1].key)
	SignerNewTheme {
		NewKeySetSelectNetworkScreenPrivate(networks, selected, {}, {}, {}, {})
	}
}
