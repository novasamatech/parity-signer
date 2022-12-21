package io.parity.signer.screens.scan

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.UniffiResult
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.isDisplayingErrorOnly
import io.parity.signer.models.storage.RepoResult
import io.parity.signer.models.storage.SeedRepository
import io.parity.signer.models.transactionIssues
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow


private const val TAG = "ScanViewModelTag"

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }


	var transactions: MutableStateFlow<List<MTransaction>> =
		MutableStateFlow(emptyList())
	var signature: MutableStateFlow<MSignatureReady?> =
		MutableStateFlow(null)
	var passwordModel: MutableStateFlow<EnterPasswordModel?> =
		MutableStateFlow(null)
	val presentableError: MutableStateFlow<String?> =
		MutableStateFlow(null)
	val errorWrongPassword = MutableStateFlow<Boolean>(false)

	private val transactionIsInProgress = MutableStateFlow<Boolean>(false)

	suspend fun performPayload(payload: String) {
		if (transactionIsInProgress.value) {
			Log.e(TAG, "started transaction while it was in progress, ignoring")
			return
		}
		transactionIsInProgress.value = true

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
				val seedNames = transactions
					.filter { it.ttype == TransactionType.SIGN }
					.mapNotNull { it.authorInfo?.address?.seedName }
				val actionResult = signTransaction("", seedNames)

				//todo scan
				//						if (actionResult.navResult.alertData != null) {
				//							Log.e(
				//								"sign error",
				//								result.navResult.alertData.toString()
				//							) //todo dmitry show error

				//password protected key, show password
				when (val modalData = actionResult?.modalData) {
					is ModalData.EnterPassword -> {
						passwordModel.value = modalData.f.toEnterPasswordModel(withShowError = false)
					}
					is ModalData.SignatureReady -> {
						signature.value = modalData.f
					}
					//ignore the rest modals
					else -> {
						//todo scan actually we won't ask for signature in this case ^^^
					}
				}
			}
			else -> {
				// Transaction with error OR
				// Transaction that does not require signing (i.e. adding network or metadata)
				// will set them below for any case and show anyway
			}
		}
		this.transactions.value = transactions
	}

	fun ifHasStateThenClear(): Boolean {
		return if (transactions.value.isNotEmpty()
			|| signature.value != null
			|| passwordModel.value != null
			|| presentableError.value != null
			|| transactionIsInProgress.value
			|| errorWrongPassword.value
		) {
			clearTransactionState()
			true
		} else false
	}

	fun clearTransactionState() {
		transactions.value = emptyList()
		signature.value = null
		passwordModel.value = null
		presentableError.value = null
		transactionIsInProgress.value = false
		errorWrongPassword.value = false
	}

	private suspend fun signTransaction(
		comment: String,
		seedNames: List<String>,
	): ActionResult? {
		return when (val phrases = seedRepository.getSeedPhrases(seedNames)) {
			is RepoResult.Failure -> {
				Log.w(TAG, "signature transactions failure ${phrases.error}")
				null
			}
			is RepoResult.Success -> backendAction(
				Action.GO_FORWARD,
				comment,
				phrases.result
			)
		}
	}

	suspend fun handlePasswordEntered(password: String) {
		val navigateResponse =
			uniffiInteractor.navigate(Action.GO_FORWARD, password)
		val actionResult =
			(navigateResponse as? UniffiResult.Success)?.result
				?: run {
			Log.e(
				TAG, "Error in entering password for a key, " +
					"navigation resp is $navigateResponse"
			)
			return
		}

		when (val modalData = actionResult.modalData) {
			// If navigation returned `enterPassword`, it means password is invalid
			is ModalData.EnterPassword -> {
				val model = modalData.f.toEnterPasswordModel(true)
				if (model.attempt > 3) {
					proceedWrongPassword()
					return
				}
				passwordModel.value = model
			}
			//password success
			is ModalData.SignatureReady -> {
				//                navigation.performFake(navigation: .init(action: .goBack))
				signature.value = modalData.f
				passwordModel.value = null
			}
			//ignore the rest modals
			else -> {
				Log.e(
					TAG, "Password is entered for transaction, but neither new password or signature is passed! Should not happen" +
						"actionResult is $actionResult"
				)
			}
		}
		// If we got `Log`, it's out of attempts to enter password
		if (actionResult.screenData is ScreenData.Log) {
			proceedWrongPassword()
		}
	}

	private fun proceedWrongPassword() {
//		navigation.performFake(navigation: .init(action: .goBack))
		//  Fake navigation to camera, as were brought back to `Log` screen on navstate error handling
//			navigation.performFake(navigation: .init(action: .navbarScan))
		errorWrongPassword.value = true
		passwordModel.value = null
	}
}
