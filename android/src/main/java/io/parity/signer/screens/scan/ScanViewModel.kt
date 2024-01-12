package io.parity.signer.screens.scan

import android.content.Context
import timber.log.Timber
import android.widget.Toast
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.R
import io.parity.signer.bottomsheets.password.EnterPasswordModel
import io.parity.signer.bottomsheets.password.toEnterPasswordModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.ScanFlowInteractor
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.storage.RepoResult
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.screens.scan.errors.LocalErrorSheetModel
import io.parity.signer.screens.scan.errors.toBottomSheetModel
import io.parity.signer.screens.scan.importderivations.ImportDerivedKeysRepository
import io.parity.signer.screens.scan.importderivations.ImportDerivedKeysRepository.ImportDerivedKeyError
import io.parity.signer.screens.scan.importderivations.allImportDerivedKeys
import io.parity.signer.screens.scan.importderivations.dominantImportError
import io.parity.signer.screens.scan.importderivations.hasImportableKeys
import io.parity.signer.screens.scan.importderivations.importableSeedKeysPreviews
import io.parity.signer.screens.scan.transaction.isDisplayingErrorOnly
import io.parity.signer.screens.scan.transaction.transactionIssues
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.DdKeySet
import io.parity.signer.uniffi.DdPreview
import io.parity.signer.uniffi.DerivedKeyError
import io.parity.signer.uniffi.MSignatureReady
import io.parity.signer.uniffi.MTransaction
import io.parity.signer.uniffi.ModalData
import io.parity.signer.uniffi.ScreenData
import io.parity.signer.uniffi.SeedKeysPreview
import io.parity.signer.uniffi.TransactionType
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.launch


private const val TAG = "ScanViewModelTag"

/**
 * Shared ViewModel for all Scan flow components, not only camera related.
 */
class ScanViewModel : ViewModel() {

	private val scanFlowInteractor = ScanFlowInteractor()
	private val uniffiInteractor: UniffiInteractor =
		ServiceLocator.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private val importKeysRepository: ImportDerivedKeysRepository by lazy {
		ImportDerivedKeysRepository(seedRepository)
	}

	data class TransactionsState(val transactions: List<MTransaction>)

	var transactions: MutableStateFlow<TransactionsState?> =
		MutableStateFlow(null)
	var signature: MutableStateFlow<MSignatureReady?> = MutableStateFlow(null)
	var bananaSplitPassword: MutableStateFlow<List<String>?> =
		MutableStateFlow(null)
	var dynamicDerivations: MutableStateFlow<DdPreview?> = MutableStateFlow(null)
	var passwordModel: MutableStateFlow<EnterPasswordModel?> =
		MutableStateFlow(null)
	val transactionError: MutableStateFlow<LocalErrorSheetModel?> =
		MutableStateFlow(null)
	val errorWrongPassword = MutableStateFlow<Boolean>(false)

	private val transactionIsInProgress = MutableStateFlow<Boolean>(false)

	suspend fun performTransactionPayload(payload: String, context: Context) {
		val fakeNavigator = FakeNavigator()
		if (transactionIsInProgress.value) {
			Timber.e(TAG, "started transaction while it was in progress, ignoring")
			return
		}
		transactionIsInProgress.value = true
		val navigateResponse = scanFlowInteractor.performTransaction(payload)

		when (navigateResponse) {
			is OperationResult.Err -> {
				transactionError.value =
					navigateResponse.error.toBottomSheetModel(context)
			}

			is OperationResult.Ok -> {
				val screenData = navigateResponse.result.screenData
				val transactions: List<MTransaction> =
					(screenData as? ScreenData.Transaction)?.f ?: run {
						Timber.e(
							TAG,
							"Error in getting transaction from qr payload, " + "screenData is $screenData, navigation resp is $navigateResponse"
						)
						clearState()
						return
					}

				// Handle transactions with just error payload
				if (transactions.all { it.isDisplayingErrorOnly() }) {
					transactionError.value = LocalErrorSheetModel(context = context,
						details = transactions.joinToString("\n") { it.transactionIssues() })
					fakeNavigator.navigate(Action.GO_BACK) //fake call
					clearState()
					return
				}

				when (transactions.firstOrNull()?.ttype) {
					TransactionType.SIGN -> {
						val seedNames =
							transactions.filter { it.ttype == TransactionType.SIGN }
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
						// We always need to `.goBack` as even if camera is dismissed without import, navigation "forward" already happened
						fakeNavigator.navigate(Action.GO_BACK)
						when (transactions.dominantImportError()) {
							DerivedKeyError.BadFormat -> {
								transactionError.value = LocalErrorSheetModel(
									title = context.getString(R.string.scan_screen_error_bad_format_title),
									subtitle = context.getString(R.string.scan_screen_error_bad_format_message),
								)
								clearState()
								return
							}

							DerivedKeyError.KeySetMissing -> {
								transactionError.value = LocalErrorSheetModel(
									title = context.getString(R.string.scan_screen_error_missing_key_set_title),
									subtitle = context.getString(R.string.scan_screen_error_missing_key_set_message),
								)
								clearState()
								return
							}

							DerivedKeyError.NetworkMissing -> {
								transactionError.value = LocalErrorSheetModel(
									title = context.getString(R.string.scan_screen_error_missing_network_title),
									subtitle = context.getString(R.string.scan_screen_error_missing_network_message),
								)
								clearState()
								return
							}

							null -> {
								//proceed, all good, now check if we need to update for derivations keys
								if (transactions.hasImportableKeys()) {
									val importDerivedKeys =
										transactions.flatMap { it.allImportDerivedKeys() }
									if (importDerivedKeys.isEmpty()) {
										this.transactions.value = TransactionsState(transactions)
									}

									when (val result =
										importKeysRepository.updateWithSeed(importDerivedKeys)) {
										is RepoResult.Success -> {
											val updatedKeys = result.result
											val newTransactionsState =
												updateTransactionsWithImportDerivations(
													transactions = transactions, updatedKeys = updatedKeys
												)
											this.transactions.value =
												TransactionsState(newTransactionsState)
										}

										is RepoResult.Failure -> {
											Toast.makeText(                        /* context = */
												context,                        /* text = */
												context.getString(R.string.import_derivations_failure_update_toast),                        /* duration = */
												Toast.LENGTH_LONG
											).show()
											clearState()
										}
									}
								} else {
									transactionError.value = LocalErrorSheetModel(
										title = context.getString(R.string.scan_screen_error_derivation_no_keys_and_no_errors_title),
										subtitle = context.getString(R.string.scan_screen_error_derivation_no_keys_and_no_errors_message),
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
		}
	}

	suspend fun performDynamicDerivationPayload(
		payload: String, context: Context
	) {
		when (val phrases = seedRepository.getAllSeeds()) {
			is RepoResult.Failure -> {
				Timber.e(
					TAG,
					"cannot get seeds to show import dynamic derivations ${phrases.error}"
				)
			}

			is RepoResult.Success -> {
				val previewDynDerivations =
					uniffiInteractor.previewDynamicDerivations(phrases.result, payload)

				when (previewDynDerivations) {
					is UniffiResult.Error -> {
						transactionError.value = LocalErrorSheetModel(
							title = context.getString(R.string.dymanic_derivation_error_custom_title),
							subtitle = previewDynDerivations.error.message ?: "",
						)
					}

					is UniffiResult.Success -> {
						dynamicDerivations.value = previewDynDerivations.result
					}
				}
			}
		}
	}

	suspend fun performDynamicDerivationTransaction(
		payload: List<String>, context: Context
	) {
		when (val phrases = seedRepository.getAllSeeds()) {
			is RepoResult.Failure -> {
				Timber.e(
					TAG,
					"cannot get seeds to show import dynamic derivations ${phrases.error}"
				)
			}

			is RepoResult.Success -> {
				val dynDerivations =
					uniffiInteractor.signDynamicDerivationsTransactions(
						phrases.result, payload
					)

				when (dynDerivations) {
					is UniffiResult.Error -> {
						transactionError.value = LocalErrorSheetModel(
							title = context.getString(R.string.scan_screen_error_derivation_no_keys_and_no_errors_title),
							subtitle = dynDerivations.error.message ?: "",
						)
					}

					is UniffiResult.Success -> {
						signature.value = dynDerivations.result.signature
						transactions.value =
							TransactionsState(dynDerivations.result.transaction)
					}
				}
			}
		}
	}

	private fun updateTransactionsWithImportDerivations(
		transactions: List<MTransaction>, updatedKeys: List<SeedKeysPreview>
	): List<MTransaction> = transactions.map { transaction ->
		if (transaction.hasImportableKeys()) {
			transaction.content.importingDerivations =
				transaction.content.importingDerivations?.map { transactionCard ->
					transactionCard.copy(
						card = when (val card = transactionCard.card) {
							is Card.DerivationsCard -> {
								card.copy(f = card.f.map { originalKey ->
									updatedKeys.firstOrNull { resultKey ->
										areSeedKeysTheSameButUpdated(originalKey, resultKey)
									} ?: originalKey
								})
							}

							else -> {
								card
								//don't update
							}
						}
					)
				}
			transaction
		} else {
			transaction
		}
	}

	fun createDynamicDerivations(
		toImport: DdKeySet, context: Context
	) {
		viewModelScope.launch {
			if (toImport.derivations.isNotEmpty()) {
				val result = importKeysRepository.createDynamicDerivationKeys(
					seedName = toImport.seedName, keysToImport = toImport.derivations
				)

				clearState()
				when (result) {
					is OperationResult.Err -> {
						val errorMessage = when (result.error) {
							is ImportDerivedKeyError.KeyNotImported -> result.error.keyToError.joinToString(
								separator = "\n"
							) {
								context.getString(
									R.string.dymanic_derivation_error_custom_message,
									it.path,
									it.errorLocalized
								)
							}

							is ImportDerivedKeyError.NoKeysImported -> result.error.errors.joinToString(
								separator = "\n"
							)

							ImportDerivedKeyError.AuthFailed -> {
								context.getString(R.string.auth_failed_message)
							}
						}
						transactionError.value = LocalErrorSheetModel(
							title = context.getString(R.string.dymanic_derivation_error_custom_title),
							subtitle = errorMessage,
						)
					}

					is OperationResult.Ok -> {
						Toast.makeText(
							context,
							context.getString(R.string.create_derivations_success),
							Toast.LENGTH_SHORT
						).show()
					}
				}
			} else {
				//list of derivations is empty
				clearState()
				Toast.makeText(
					context,
					context.getString(R.string.create_derivations_empty),
					Toast.LENGTH_SHORT
				).show()
			}
		}
	}

	private fun areSeedKeysTheSameButUpdated(
		originalKey: SeedKeysPreview, resultKey: SeedKeysPreview
	): Boolean =
		originalKey.name == resultKey.name && resultKey.derivedKeys.all { derKey ->
			originalKey.derivedKeys.any { it.derivationPath == derKey.derivationPath }
		}

	fun onImportKeysTap(transactions: TransactionsState, context: Context) {
		val importableKeys =
			transactions.transactions.flatMap { it.importableSeedKeysPreviews() }

		val importResult = importKeysRepository.importDerivedKeys(importableKeys)
		val derivedKeysCount = importableKeys.sumOf { it.derivedKeys.size }

		when (importResult) {
			is RepoResult.Success -> {
				Toast.makeText(          /* context = */ context,          /* text = */
					context.resources.getQuantityString(
						R.plurals.import_derivations_success_keys_imported,
						derivedKeysCount,
						derivedKeysCount,
					), /* duration = */
					Toast.LENGTH_LONG
				).show()
			}

			is RepoResult.Failure -> {
				Toast.makeText(          /* context = */ context,          /* text = */
					context.getString(R.string.import_derivations_failure_toast),          /* duration = */
					Toast.LENGTH_LONG
				).show()
			}
		}

		this.transactions.value = null
	}


	fun ifHasStateThenClear(): Boolean {
		return if (transactions.value != null || signature.value != null || passwordModel.value != null || transactionError.value != null || transactionIsInProgress.value || errorWrongPassword.value || bananaSplitPassword.value != null || dynamicDerivations.value != null) {
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
		dynamicDerivations.value = null
		transactionError.value = null
		transactionIsInProgress.value = false
		errorWrongPassword.value = false
	}

	private suspend fun signTransaction(
		comment: String,
		seedNames: List<String>,
	): ActionResult? {
		return when (val phrases = seedRepository.getSeedPhrases(seedNames)) {
			is RepoResult.Failure -> {
				Timber.w(TAG, "signature transactions failure ${phrases.error}")
				null
			}

			is RepoResult.Success -> {
				scanFlowInteractor.continueSigningTransaction(
					comment,
					phrases.result,
				)
			}
		}
	}

	suspend fun handlePasswordEntered(password: String) {
		val navigateResponse =
			scanFlowInteractor.handlePasswordEntered(password)
		val actionResult =
			(navigateResponse as? OperationResult.Ok)?.result ?: run {
				Timber.e(
					TAG,
					"Error in entering password for a key, " + "navigation resp is $navigateResponse"
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
				Timber.e(
					TAG,
					"Password is entered for transaction, but neither new password or signature is passed! Should not happen" + "actionResult is $actionResult"
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
		val fakeNavigator = FakeNavigator()
		// Dismissing password modal goes to `Log` screen
		fakeNavigator.backAction()
		// Pretending to navigate back to `Scan` so navigation states for new QR code scan will work
		fakeNavigator.navigate(Action.NAVBAR_SCAN, "", "")
	}
}
