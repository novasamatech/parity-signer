package io.parity.signer.screens.scan

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.lifecycle.ViewModel
import io.parity.signer.R
import io.parity.signer.backend.UniffiResult
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.FakeNavigator
import io.parity.signer.models.storage.RepoResult
import io.parity.signer.models.storage.SeedRepository
import io.parity.signer.screens.scan.elements.PresentableErrorModel
import io.parity.signer.screens.scan.importderivations.*
import io.parity.signer.screens.scan.transaction.isDisplayingErrorOnly
import io.parity.signer.screens.scan.transaction.transactionIssues
import io.parity.signer.uniffi.*
import kotlinx.coroutines.flow.MutableStateFlow


private const val TAG = "ScanViewModelTag"

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private val importKeysService: ImportDerivedKeysRepository by lazy {
		ImportDerivedKeysRepository(
			seedRepository
		)
	}

	data class TransactionsState(
		val transactions: List<MTransaction>,
	)

	var transactions: MutableStateFlow<TransactionsState?> =
		MutableStateFlow(null)
	var signature: MutableStateFlow<MSignatureReady?> =
		MutableStateFlow(null)
	var bananaSplitPassword: MutableStateFlow<List<String>?> =
		MutableStateFlow(null)
	var passwordModel: MutableStateFlow<EnterPasswordModel?> =
		MutableStateFlow(null)
	val presentableError: MutableStateFlow<PresentableErrorModel?> =
		MutableStateFlow(null)
	val errorWrongPassword = MutableStateFlow<Boolean>(false)

	private val transactionIsInProgress = MutableStateFlow<Boolean>(false)

	suspend fun performPayload(payload: String, context: Context) {
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
			presentableError.value = PresentableErrorModel(
				details = transactions.joinToString("\n") { it.transactionIssues() }
			)
			uniffiInteractor.navigate(Action.GO_BACK) //fake call
			return
		}

		when (transactions.firstOrNull()?.ttype) {
			TransactionType.SIGN -> {
				val seedNames = transactions
					.filter { it.ttype == TransactionType.SIGN }
					.mapNotNull { it.authorInfo?.address?.seedName }
				val actionResult = signTransaction("", seedNames)

				//password protected key, show password
				when (val modalData = actionResult?.modalData) {
					is ModalData.EnterPassword -> {
						passwordModel.value =
							modalData.f.toEnterPasswordModel(withShowError = false)
					}
					is ModalData.SignatureReady -> {
						signature.value = modalData.f
					}
					//ignore the rest modals
					else -> {
						//actually we won't ask for signature in this case ^^^
//						we can get result.navResult.alertData with error from Rust but it's not in new design
					}
				}
				this.transactions.value = TransactionsState(transactions)
			}
			TransactionType.IMPORT_DERIVATIONS -> {
				val fakeNavigator = FakeNavigator()
				// We always need to `.goBack` as even if camera is dismissed without import, navigation "forward" already happened
				fakeNavigator.navigate(Action.GO_BACK)
				when (transactions.dominantImportError()) {
					DerivedKeyError.BadFormat -> {
						presentableError.value = PresentableErrorModel(
							title = context.getString(R.string.scan_screen_error_bad_format_title),
							message = context.getString(R.string.scan_screen_error_bad_format_message),
						)
						return
					}
					DerivedKeyError.KeySetMissing -> {
						presentableError.value = PresentableErrorModel(
							title = context.getString(R.string.scan_screen_error_missing_key_set_title),
							message = context.getString(R.string.scan_screen_error_missing_key_set_message),
						)
						return
					}
					DerivedKeyError.NetworkMissing -> {
						presentableError.value = PresentableErrorModel(
							title = context.getString(R.string.scan_screen_error_missing_network_title),
							message = context.getString(R.string.scan_screen_error_missing_network_message),
						)
						return
					}
					null -> {
						//proceed, no full error
						if (transactions.hasImportableKeys()) {
							this.transactions.value = TransactionsState(transactions)
						} else {
							presentableError.value = PresentableErrorModel(
								title = context.getString(R.string.scan_screen_error_key_already_exists_title),
								message = context.getString(R.string.scan_screen_error_key_already_exists_message),
							)
							return
						}
					}
				}
			}
			else -> {
				// Transaction with error OR
				// Transaction that does not require signing (i.e. adding network or metadata)
				// will set them below for any case and show anyway
				this.transactions.value = TransactionsState(transactions)
			}
			//handle alert error rust/navigator/src/navstate.rs:396
		}
	}

	fun onImportKeysTap(transactions: TransactionsState, context: Context) {
		val importableKeys =
			transactions.transactions.flatMap { it.importableSeedKeysPreviews() }

		val importResult = importKeysService.importDerivedKeys(importableKeys)
		val derivedKeysCount =
			transactions.transactions.sumOf { it.importableKeysCount() }
		when (importResult) {
			is RepoResult.Success -> {
				Toast.makeText(
					/* context = */ context,
					/* text = */ context.resources.getQuantityString(
						R.plurals.import_derivations_success_keys_imported,
						derivedKeysCount,
						derivedKeysCount,
					), /* duration = */ Toast.LENGTH_LONG
				).show()
			}
			is RepoResult.Failure -> {
				Toast.makeText(
					/* context = */ context,
					/* text = */
					context.getString(R.string.import_derivations_failure_toast),
					/* duration = */
					Toast.LENGTH_LONG
				).show()
			}
		}

		this.transactions.value = null
	}


	fun ifHasStateThenClear(): Boolean {
		return if (
			transactions.value != null
			|| signature.value != null
			|| passwordModel.value != null
			|| presentableError.value != null
			|| transactionIsInProgress.value
			|| errorWrongPassword.value
			|| bananaSplitPassword.value != null
		) {
			clearState()
			true
		} else {
			false
		}
	}

	fun clearState() {
		transactions.value = null
		signature.value = null
		passwordModel.value = null
		bananaSplitPassword.value = null
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
					TAG,
					"Password is entered for transaction, but neither new password or signature is passed! Should not happen" +
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
		errorWrongPassword.value = true
		passwordModel.value = null
		resetRustModalToNewScan()
	}

	fun resetRustModalToNewScan() {
		// Dismissing password modal goes to `Log` screen
		backendAction(Action.GO_BACK, "", "")
		// Pretending to navigate back to `Scan` so navigation states for new QR code scan will work
		backendAction(Action.NAVBAR_SCAN, "", "")
	}
}
