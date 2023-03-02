package io.parity.signer.backend

import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.submitErrorState
import io.parity.signer.domain.toNetworkModel
import io.parity.signer.screens.scan.errors.TransactionError
import io.parity.signer.uniffi.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.collectLatest

/**
 * Wrapper for uniffi calls into rust. Made for centralized handling errors
 * and to have those functions scoped in specific namespace
 */
class UniffiInteractor() {

	/**
	 * Rust db is initializing only when main screen is shown
	 * and we need to provide seeds for it, so we cannot do it rightaway.
	 */
	val wasRustInitialized = MutableStateFlow(false)

	private val suspendedTasksContext: CoroutineScope =
		CoroutineScope(Dispatchers.IO)

	suspend fun navigate(
		action: Action,
		details: String = "",
		seedPhrase: String = "",
	): UniffiResult<ActionResult> = withContext(Dispatchers.IO) {
		try {
			UniffiResult.Success(backendAction(action, details, seedPhrase))
		} catch (e: ErrorDisplayed) {
			UniffiResult.Error(e)
		}
	}

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

	suspend fun exportSeedKeyInfos(seedsToExport: List<String>): UniffiResult<MKeysInfoExport> =
		withContext(Dispatchers.IO) {
			try {
				val keyInfo = exportKeyInfo(
					selectedNames = seedsToExport.associateWith { ExportedSet.All },
				)
				UniffiResult.Success(keyInfo)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
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
			val keyInfo = exportKeyInfo(

				selectedNames = mapOf(seed to ExportedSet.Selected(pathAndNetworks)),
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
						encodeToQr(it, false)
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
					io.parity.signer.uniffi.getAllNetworks().map { it.toNetworkModel() }
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
}

sealed class UniffiResult<T> {
	data class Success<T>(val result: T) : UniffiResult<T>()
	data class Error<Any>(val error: ErrorDisplayed) : UniffiResult<Any>()
}

sealed class BackendNavigationResult {
	data class Success(val result: ActionResult) : BackendNavigationResult()
	data class Error(val error: TransactionError) : BackendNavigationResult()
}

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

