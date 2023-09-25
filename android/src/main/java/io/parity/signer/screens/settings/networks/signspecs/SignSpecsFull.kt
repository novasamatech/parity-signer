package io.parity.signer.screens.settings.networks.signspecs

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.domain.Callback
import io.parity.signer.screens.settings.networks.signspecs.view.SignSpecsListScreen
import io.parity.signer.screens.settings.networks.signspecs.view.SignSpecsResultBottomSheet
import io.parity.signer.ui.BottomSheetWrapperRoot


@Composable
fun SignSpecsFull(
	sc: SignSpecsListModel,
	onBack: Callback
) {
	val vm: SignSpecsViewModel = viewModel()

	val passwordState = vm.password.collectAsStateWithLifecycle()
	val signatureState = vm.signature.collectAsStateWithLifecycle()

	val backAction = {
		val wasState = vm.isHasStateThenClear()
		if (!wasState) onBack()
	}
	BackHandler(onBack = backAction)

	SignSpecsListScreen(
		model = sc,
		onBack = onBack,
		signSufficientCrypto = vm::onSignSufficientCrypto,
		modifier = Modifier.statusBarsPadding(),
	)

	passwordState.value?.let { enterPasswordModel ->
		BottomSheetWrapperRoot(onClosedAction = vm::isHasStateThenClear) {
			EnterPassword(
				data = enterPasswordModel,
				proceed = { password ->
					vm.passwordAttempt(password)
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

