package io.parity.signer.screens.scan

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.*
import io.parity.signer.models.getSeed
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow


private const val TAG = "ScanViewModelTag"

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendLocator.uniffiInteractor
	private val authentication = ServiceLocator.authentication


	var pendingTransactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())
	var signature: MutableStateFlow<MSignatureReady?> =
		MutableStateFlow(null)

	private val transactionInProgress = MutableStateFlow<Boolean>(false)
	val presentableError = MutableStateFlow<String?>(null)

	suspend fun performPayloads(payloads: Set<String>): List<MTransaction> {//todo remove?
		val allResults = payloads.map { payload ->
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		}
		//todo handle error cases and show ot user?
		allResults.filterIsInstance<UniffiResult.Error<Any>>().forEach { error ->
			Log.e(
				TAG,
				"Camera scan: transaction parsing failed, ${error.error.message}"
			)
		}
		return allResults.filterIsInstance<UniffiResult.Success<ActionResult>>()
			.mapNotNull { (it.result.screenData as? ScreenData.Transaction)?.f }
			.flatten()
	}

	suspend fun performPayload(payload: String) {
		if (transactionInProgress.value) {
			Log.e(TAG, "started transaction while it was in progress, ignoring")
			return
		}
		transactionInProgress.value = true

		val navigateResponse =
			uniffiInteractor.navigate(Action.TRANSACTION_FETCHED, payload)
		val screenData =
			(navigateResponse as? UniffiResult.Success)?.result?.screenData
		val transactions = (screenData as? ScreenData.Transaction)?.f
			?: run {
				Log.e(
					TAG, "Error in getting transaction from qr payload, " +
						"screenData is $screenData, navigation resp is $navigateResponse"
				)
				return
			}

		// Handle transactions with just error payload
		if (transactions.all { it.isDisplayingErrorOnly() }) {
			presentableError.value =
				transactions.joinToString("\n") { it.transactionIssues() }

			uniffiInteractor.navigate(Action.GO_BACK)
			return
		}

		when (transactions.firstOrNull()?.ttype) {
			TransactionType.SIGN -> {
//				val actionResult todo from new data model

//				                let actionResult = sign(transactions: transactions)
//                self.transactions = transactions
//                // Password protected key, continue to modal
//                if case let .enterPassword(value) = actionResult.modalData {
//                    enterPassword = value
//                    isPresentingEnterPassword = true
//                }
//                // Transaction ready to sign
//                if case let .signatureReady(value) = actionResult.modalData {
//                    signature = value
//                    continueWithSignature()
//                }
			}
			else -> {
				// Transaction with error
				// Transaction that does not require signing (i.e. adding network or metadata)
				//show transactions
			}
		}

	}
	fun clearTransactionState() {
		pendingTransactions.value = emptyList()
		signature.value = null
		transactionInProgress.value = false
	}

	//				scope.launch {
	//					val result = transactionVm.signTransaction(
	//						comment = comment.value,
	//						seedNames = transactions.mapNotNull { it.authorInfo?.address?.seedName },
	//						signerVM = signerDataModel,
	//					)
	//					//todo dmitry handle non happy cases as well and probably in viewmodel not here
	//					if (result is SignResult.Success) {
	//						if (result.navResult.alertData != null) {
	//							Log.e(
	//								"sign error",
	//								result.navResult.alertData.toString()
	//							) //todo dmitry show error
	//						} else if (result.navResult.modalData != null) {
	//							if (result.navResult.modalData is ModalData.SignatureReady) {
	//								onSigReady((result.navResult.modalData as ModalData.SignatureReady).f)
	//							} else {
	//								//todo dmitry show password
	//								Log.e(
	//									"sign modal is not handled",
	//									result.navResult.modalData.toString()
	//								)
	//							}
	//						} else {
	//							(result.navResult.screenData as? ScreenData.Transaction)?.f?.let { transactions ->
	////								onSuccess(transactions)
	////									screenTransactions = transactions
	//							}
	//						}
	//					}
	//				}
	private suspend fun signTransaction(
		comment: String,
		seedNames: List<String>,
		signerVM: SignerDataModel, //todo dmitry inbound get seed from it!
	): SignResult {
		return when (val authResult =
			//todo check how to check if user is already authenticated with biometric prompt
			authentication.authenticate(signerVM.activity)) {
			AuthResult.AuthSuccess -> {
				val seedPhrases = seedNames
					.map { signerVM.getSeed(it) }
					.filter { it.isNotEmpty() }
					.joinToString(separator = "\n")

				if (seedPhrases.isNotBlank()) {
					SignResult.Success(
						backendAction(Action.GO_FORWARD, comment, seedPhrases)
					)
				} else {
					SignResult.Failure(null)
				}
			}
			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				SignResult.Failure(authResult)
			}
		}
	}

	sealed class SignResult {
		data class Success(val navResult: ActionResult) : SignResult()
		data class Failure(val auth: AuthResult?) : SignResult()
	}

}
