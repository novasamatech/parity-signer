package io.parity.signer.screens.settings.networks.signspecs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.components.sharedcomponents.KeyCardModelBase
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.errors.LocalErrorBottomSheet
import io.parity.signer.screens.settings.networks.signspecs.view.SignSpecsListScreen
import io.parity.signer.screens.settings.networks.signspecs.view.SignSpecsResultBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun SignSpecsFull(
	model: SignSpecsListModel,
	inputData: SignSpecsInput,
	onBack: Callback
) {
	val vm: SignSpecsViewModel = viewModel()

	val passwordState = vm.password.collectAsStateWithLifecycle()
	val signatureState = vm.signature.collectAsStateWithLifecycle()
	val errorState = vm.localError.collectAsStateWithLifecycle()

	val backAction = {
		val wasState = vm.isHasStateThenClear()
		if (!wasState) onBack()
	}
	BackHandler(onBack = backAction)

	SignSpecsListScreen(
		model = model,
		onBack = onBack,
		signSufficientCrypto = { key: KeyCardModelBase, addressKey64: String, hasPassword: Boolean ->
			if (hasPassword) {
				vm.requestPassword(key, addressKey64)
			} else {
				vm.onSignSpecs(inputData, key, addressKey64, null)
			}
		},
		modifier = Modifier.statusBarsPadding(),
	)

	errorState.value?.let { error ->
		BottomSheetWrapperRoot(onClosedAction = vm::clearError) {
			LocalErrorBottomSheet(
				error = error,
				onOk = vm::clearError,
			)
		}
	} ?: passwordState.value?.let { enterPasswordModel ->
		BottomSheetWrapperRoot(onClosedAction = vm::isHasStateThenClear) {
			EnterPassword(
				data = enterPasswordModel.model,
				proceed = { password ->
					vm.onSignSpecs(
						input = inputData,
						keyModel = enterPasswordModel.model.keyCard,
						addressKey = enterPasswordModel.addressKey64,
						password = password
					)
				},
				onClose = vm::isHasStateThenClear,
			)
		}
	} ?: signatureState.value?.let { signature ->
		BottomSheetWrapperRoot(onClosedAction = vm::isHasStateThenClear) {
			SignSpecsResultBottomSheet(
				model = signature,
				onBack = vm::isHasStateThenClear,
			)
		}
	}
}

