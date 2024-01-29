package io.parity.signer.domain.backend

import android.content.Context
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.KeySetDetailsModel
import io.parity.signer.domain.KeySetsListModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.VerifierDetailsModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.toKeySetDetailsModel
import io.parity.signer.domain.toKeySetsSelectModel
import io.parity.signer.domain.toNetworkModel
import io.parity.signer.domain.toVerifierDetailsModel
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel
import io.parity.signer.screens.keydetails.exportprivatekey.toPrivateKeyExportModel
import io.parity.signer.screens.settings.networks.details.NetworkDetailsModel
import io.parity.signer.screens.settings.networks.details.toNetworkDetailsModel
import io.parity.signer.uniffi.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest
import java.lang.RuntimeException

/**
 * Wrapper for uniffi calls into rust. Made for centralized handling errors
 * and to have those functions scoped in specific namespace
 */
class UniffiInteractor(val appContext: Context) {

	/**
	 * Rust db is initializing only when main screen is shown
	 * and we need to provide seeds for it, so we cannot do it rightaway.
	 */
	val wasRustInitialized = MutableStateFlow(false)

	private val suspendedTasksContext: CoroutineScope =
		CoroutineScope(Dispatchers.IO)

	fun historyDeviceWasOnline() {
		if (wasRustInitialized.value) {
			io.parity.signer.uniffi.historyDeviceWasOnline()
		} else {
			suspendedTasksContext.launch {
				wasRustInitialized.collectLatest {
					if (it) {
						io.parity.signer.uniffi.historyDeviceWasOnline()
						return@collectLatest
					}
				}
			}
		}
	}

	suspend fun exportSeedWithKeys(
		seed: String, derivedKeyAddr: List<String>
	): UniffiResult<MKeysInfoExport> = withContext(Dispatchers.IO) {
		try {
			val keys = keysBySeedName(seed)
			val pathAndNetworks = derivedKeyAddr.map { keyAddr ->
				val key = keys.set.find { it.key.addressKey == keyAddr }!!
				PathAndNetwork(
					key.key.address.path, key.network.networkSpecsKey
				)
			}
			val keyInfo = io.parity.signer.uniffi.exportKeyInfo(
				seedName = seed,
				exportedSet = ExportedSet.Selected(pathAndNetworks),
			)
			UniffiResult.Success(keyInfo)
		} catch (e: ErrorDisplayed) {
			UniffiResult.Error(e)
		}
	}

	suspend fun encodeToQrImages(binaryData: List<List<UByte>>): UniffiResult<List<List<UByte>>> =
		withContext(Dispatchers.IO) {
			try {
				val images = binaryData.map {
					async(Dispatchers.IO) {
						io.parity.signer.uniffi.encodeToQr(it, false)
					}
				}.map { it.await() }
				UniffiResult.Success(images)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getAllNetworks(): UniffiResult<List<NetworkModel>> =
		withContext(Dispatchers.IO) {
			try {
				val networks =
					io.parity.signer.uniffi.getManagedNetworks().networks
						.map { it.toNetworkModel() }
				UniffiResult.Success(networks)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun validateDerivationPath(
		path: String,
		seed: String,
		selectedNetworkSpecs: String
	): UniffiResult<DerivationCheck> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult = substratePathCheck(
					seedName = seed,
					path = path,
					network = selectedNetworkSpecs
				)
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getLogs(): UniffiResult<MLog> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult = io.parity.signer.uniffi.getLogs()
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getLogDetails(logIndex: UInt): UniffiResult<MLogDetails> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult = io.parity.signer.uniffi.getLogDetails(logIndex)
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun clearLogHistory(): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult = io.parity.signer.uniffi.clearLogHistory()
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun addCommentToLogs(userComment: String): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult =
					io.parity.signer.uniffi.handleLogComment(userComment)
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun previewDynamicDerivations(
		seeds: Map<String, String>,
		payload: String
	): UniffiResult<DdPreview> =
		withContext(Dispatchers.IO) {
			try {
				val validationResult =
					io.parity.signer.uniffi.previewDynamicDerivations(seeds, payload)
				UniffiResult.Success(validationResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun signDynamicDerivationsTransactions(
		seeds: Map<String, String>,
		payload: List<String>
	): UniffiResult<MSignedTransaction> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.signDdTransaction(payload, seeds)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun createNewSeedPhrase(
	): UniffiResult<String> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult = io.parity.signer.uniffi.printNewSeed("")
				UniffiResult.Success(transactionResult.seedPhrase)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun generateSecretKeyQr(
		publicKey: String,
		expectedSeedName: String,
		networkSpecsKey: String,
		seedPhrase: String,
		keyPassword: String?,
	): UniffiResult<PrivateKeyExportModel> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.generateSecretKeyQr(
						publicKey = publicKey,
						expectedSeedName = expectedSeedName,
						networkSpecsKey = networkSpecsKey,
						seedPhrase = seedPhrase,
						keyPassword = keyPassword,
					).toPrivateKeyExportModel()
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getKeySets(
		seedNames: List<String>
	): UniffiResult<KeySetsListModel> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.getSeeds(seedNames).toKeySetsSelectModel()
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getKeyPublicKey(
		addressKey: String,
		networkSpecsKey: String
	): UniffiResult<MKeyDetails> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.getKeySetPublicKey(
						addressKey,
						networkSpecsKey
					)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun removeKeySet(
		keySetName: String
	): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult = io.parity.signer.uniffi.removeKeySet(keySetName)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun getManagedNetworkDetails(
		networkKey: String
	): UniffiResult<NetworkDetailsModel> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.getManagedNetworkDetails(networkKey)
						.toNetworkDetailsModel()
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun removeMetadataManagedNetwork(
		networkKey: String,
		metadataSpecsVersion: String
	): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.removeMetadataOnManagedNetwork(
						networkKey,
						metadataSpecsVersion
					)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun removeManagedNetwork(
		networkKey: String,
	): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.removeManagedNetwork(
						networkKey,
					)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun keySetBySeedName(seedName: String): OperationResult<KeySetDetailsModel, ErrorDisplayed> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.keysBySeedName(seedName)
						.toKeySetDetailsModel()
				transactionResult
			} catch (e: ErrorDisplayed) {
				OperationResult.Err(e)
			}
		}

	suspend fun getVerifierDetails(): UniffiResult<VerifierDetailsModel> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.getVerifierDetails().toVerifierDetailsModel()
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun removedDerivedKey(
		addressKey: String,
		networkSpecsKey: String,
	): UniffiResult<Unit> =
		withContext(Dispatchers.IO) {
			try {
				val transactionResult =
					io.parity.signer.uniffi.removeDerivedKey(addressKey, networkSpecsKey)
				UniffiResult.Success(transactionResult)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}
}

sealed class UniffiResult<T> {
	data class Success<T>(val result: T) : UniffiResult<T>()
	data class Error<Any>(val error: ErrorDisplayed) : UniffiResult<Any>()
}

sealed class OperationResult<out T, out E> {
	data class Ok<out T>(val result: T) : OperationResult<T, Nothing>()
	data class Err<out E>(val error: E) : OperationResult<Nothing, E>()
}

sealed interface AuthOperationResult{
	data object Success: AuthOperationResult
	data class Error(val exception: Exception): AuthOperationResult
	data class AuthFailed(val result: AuthResult) : AuthOperationResult

}

sealed class CompletableResult<out T, out E> {
	data class Ok<out T>(val result: T) : CompletableResult<T, Nothing>()
	data class Err<out E>(val error: E) : CompletableResult<Nothing, E>()
	object InProgress : CompletableResult<Nothing, Nothing>()
}

@Deprecated("Handle error state")
fun <T> UniffiResult<T>.mapError(): T? {
	return when (this) {
		is UniffiResult.Error -> {
			submitErrorState("uniffi interaction exception $error")
			null
		}

		is UniffiResult.Success -> {
			result
		}
	}
}

/**
 * Dangerous cast to non error so we can see a reason in crash logs
 */
@Deprecated("Handle error state")
fun <T> UniffiResult<T>.mapErrorForce(): T {
	return when (this) {
		is UniffiResult.Error -> {
			throw RuntimeException("uniffi interaction exception $error")
		}

		is UniffiResult.Success -> {
			result
		}
	}
}

fun <T> UniffiResult<T>.toOperationResult(): OperationResult<T, ErrorDisplayed> {
	return when (this) {
		is UniffiResult.Error -> OperationResult.Err(this.error)
		is UniffiResult.Success -> OperationResult.Ok(this.result)
	}
}

@Deprecated("Handle error state")
fun <T, V> OperationResult<T, V>.mapError(): T? {
	return when (this) {
		is OperationResult.Err -> {
			submitErrorState("operation interaction exception $error")
			null
		}

		is OperationResult.Ok -> {
			result
		}
	}
}

fun <T, V, R> OperationResult<T, V>.mapInner(transform: (T) -> R): OperationResult<R, V> =
	when (this) {
		is OperationResult.Err -> this
		is OperationResult.Ok -> OperationResult.Ok(transform(this.result))
	}

fun <T, V, R> OperationResult<T, V>.map(transform: (T) -> OperationResult<R, V>): OperationResult<R, V> =
	when (this) {
		is OperationResult.Err -> this
		is OperationResult.Ok -> transform(this.result)
	}
