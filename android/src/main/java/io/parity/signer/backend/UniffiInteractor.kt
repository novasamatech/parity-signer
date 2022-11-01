package io.parity.signer.backend

import io.parity.signer.models.submitErrorState
import io.parity.signer.uniffi.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.withContext

/**
 * Wrapper for uniffi calls into rust. Made for centralized handling errors
 * and to have those functions scoped in specific namespace
 */
class UniffiInteractor(private val dbName: String) {

	suspend fun navigate(
		action: Action,
		details: String,
		seedPhrase: String
	): UniffiResult<ActionResult> = withContext(Dispatchers.IO) {
		try {
			UniffiResult.Success(backendAction(action, details, seedPhrase))
		} catch (e: ErrorDisplayed) {
			UniffiResult.Error(e)
		}
	}

	suspend fun exportSeedKeyInfos(seedsToExport: List<String>): UniffiResult<MKeysInfoExport> =
		withContext(Dispatchers.IO) {
			try {
				val keyInfo = exportKeyInfo(
					dbname = dbName,
					selectedNames = seedsToExport.associateWith { ExportedSet.All },
				)
				UniffiResult.Success(keyInfo)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}

	suspend fun exportSeedWithKeys(
		seed: String,
		derivationsAndNetworkSpecs: List<Pair<String,String>>
	): UniffiResult<MKeysInfoExport> =
		withContext(Dispatchers.IO) {
			try {
				val keyInfo = exportKeyInfo(
					dbname = dbName,
					selectedNames = mapOf(seed to ExportedSet.Selected(
						s = derivationsAndNetworkSpecs.map { PathAndNetwork(it.first, it.second) }
					)),
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
						encodeToQr(it)
					}
				}.map { it.await() }
				UniffiResult.Success(images)
			} catch (e: ErrorDisplayed) {
				UniffiResult.Error(e)
			}
		}
}

sealed class UniffiResult<T> {
	data class Success<T>(val result: T) : UniffiResult<T>()
	data class Error<Any>(val error: ErrorDisplayed) : UniffiResult<Any>()
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
