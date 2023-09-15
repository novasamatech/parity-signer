package io.parity.signer.screens.settings.networks.sufficientcrypto

import androidx.compose.runtime.Composable
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.screens.settings.networks.sufficientcrypto.view.SufficientCryptoReadyBottomSheet
import io.parity.signer.bottomsheets.password.EnterPassword
import io.parity.signer.screens.settings.networks.sufficientcrypto.view.SignSufficientCryptoScreen
import io.parity.signer.uniffi.MSignSufficientCrypto


//todo dmitry implement
//				as SignSufficientCryptoInteractor done
//				navstate.rs:830 it's Sign sufficient crypto


//		SignSufficientCrypto(
//			screenData.f,
//			sharedViewModel::signSufficientCrypto
//		)
// end of action here calling go forward and it's in navstate.rs:427
//			todo dmitry handle password here on action
//			io/parity/signer/domain/storage/TransactionOld.kt:8 ^^

//todo dmitry get this model like in
// ios/PolkadotVault/Backend/NavigationServices/ManageNetworkDetailsService.swift:10


@Composable
fun SignSufficientCryptoFull(sc: MSignSufficientCrypto) {
	val vm: SignSufficientCryptoViewModel = viewModel()

	val passwordState = vm.password.collectAsStateWithLifecycle()
	val signatureState = vm.signature.collectAsStateWithLifecycle()

	SignSufficientCryptoScreen(
		model = sc,
		signSufficientCrypto = vm::onSignSufficientCrypto,
	)

	passwordState.value?.let { enterPasswordModel ->
		EnterPassword(
			data = enterPasswordModel,
			proceed = {
//								todo dmitry
			},
			onClose = {
//								todo dmitry
			},
		)
	} ?: signatureState.value?.let { signature ->
			SufficientCryptoReadyBottomSheet(
				sufficientCrypto = signature,
			)
	}

}

