package io.parity.signer.domain.storage

import android.security.keystore.UserNotAuthenticatedException
import timber.log.Timber
import android.widget.Toast
import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.backend.AuthOperationResult
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiInteractor
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.domain.submitErrorState
import io.parity.signer.uniffi.ErrorDisplayed
import io.parity.signer.uniffi.createKeySet


class SeedRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
	private val uniffiInteractor: UniffiInteractor,
) {

	fun containSeedName(seedName: String): Boolean {
		return storage.lastKnownSeedNames.value.contains(seedName)
	}

	fun getLastKnownSeedNames(): Array<String> {
		return storage.lastKnownSeedNames.value
	}

	suspend fun getAllSeeds(): RepoResult<Map<String, String>> {
		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				val result = storage.getSeedNames()
					.associateWith { seedName -> storage.getSeed(seedName, false) }
				RepoResult.Success(result)
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				RepoResult.Failure(RuntimeException("auth error - $authResult"))
			}
		}
	}

	/**
	 * Try to get phrases if timeout - request auth
	 */
	suspend fun getSeedPhrases(seedNames: List<String>): RepoResult<String> {
		return try {
			try {
				getSeedPhrasesDangerous(seedNames)
			} catch (e: UserNotAuthenticatedException) {
				when (val authResult =
					authentication.authenticate(activity)) {
					AuthResult.AuthSuccess -> {
						getSeedPhrasesDangerous(seedNames)
					}

					AuthResult.AuthError,
					AuthResult.AuthFailed,
					AuthResult.AuthUnavailable -> {
						RepoResult.Failure(RuntimeException("auth error - $authResult"))
					}
				}
			}
		} catch (e: java.lang.Exception) {
			Timber.d("get seed failure", e.toString())
			Toast.makeText(activity, "get seed failure: $e", Toast.LENGTH_LONG).show()
			RepoResult.Failure(RuntimeException("Unexpected Exception", e))
		}
	}

	/**
	 * Force ask for authentication and get seed phrase
	 * This does not work with runBlocking() !
	 */
	suspend fun getSeedPhraseForceAuth(seedName: String): RepoResult<String> {
		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				getSeedPhrasesDangerous(listOf(seedName))
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				RepoResult.Failure(RuntimeException("auth error - $authResult"))
			}
		}
	}

	suspend fun fillSeedToPhrasesAuth(seedNames: List<String>): RepoResult<List<Pair<String, String>>> {
		return try {
			when (val authResult =
				authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> {
					val result = seedNames.map { it to storage.getSeed(it) }
					return if (result.any { it.second.isEmpty() }) {
						RepoResult.Failure(IllegalStateException("phrase some are empty - broken storage?"))
					} else {
						RepoResult.Success(result)
					}
				}

				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					RepoResult.Failure(RuntimeException("auth error - $authResult"))
				}
			}
		} catch (e: java.lang.Exception) {
			Timber.d("get seed failure", e.toString())
			Toast.makeText(activity, "get seed failure: $e", Toast.LENGTH_LONG).show()
			RepoResult.Failure(RuntimeException("Unexpected Exception", e))
		}
	}

	suspend fun addSeed(
		seedName: String,
		seedPhrase: String,
		networksKeys: List<String>
	): AuthOperationResult {
		if (isSeedPhraseCollision(seedPhrase)) {
			return AuthOperationResult.Error(Exception("Seed Phrase Collision - can't proceed"))
		}

		try {
			addSeedDangerous(seedName, seedPhrase, networksKeys)
			return AuthOperationResult.Success
		} catch (e: UserNotAuthenticatedException) {
			return when (val authResult = authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> {
					addSeedDangerous(seedName, seedPhrase, networksKeys)
					AuthOperationResult.Success
				}

				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Timber.w(TAG, "auth error - $authResult")
					AuthOperationResult.AuthFailed(authResult)
				}
			}
		} catch (e: java.lang.Exception) {
			Timber.e(TAG, e.toString())
			return AuthOperationResult.Error(e)
		}
	}

	private fun addSeedDangerous(
		seedName: String,
		seedPhrase: String,
		networks: List<String>,
	) {
		storage.addSeed(seedName, seedPhrase)
		try {
			createKeySet(seedName, seedPhrase, networks)
		} catch (e: ErrorDisplayed) {
			submitErrorState("error in add seed $e")
		}
	}

	/**
	 * All logic required to remove seed from memory
	 *
	 * 1. Remover encrypted storage item
	 * 2. Synchronizes list of seeds with rust
	 * 3. Calls rust remove seed logic
	 */
	suspend fun removeKeySet(seedName: String): OperationResult<Unit, Exception> {
		return when (val authResult = authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				try {
					storage.removeSeed(seedName)
					when (val remove = uniffiInteractor.removeKeySet(seedName)) {
						is UniffiResult.Error -> OperationResult.Err(remove.error)
						is UniffiResult.Success -> OperationResult.Ok(Unit)
					}
				} catch (e: java.lang.Exception) {
					Timber.d("remove seed error", e.toString())
					OperationResult.Err(e)
				}
			}

			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				Timber.d("remove seed auth error ", authResult.toString())
				OperationResult.Err(Exception("remove seed auth error $authResult"))
			}
		}
	}

	private fun getSeedPhrasesDangerous(seedNames: List<String>): RepoResult<String> {
		val seedPhrases = seedNames
			.map { storage.getSeed(it) }
			.filter { it.isNotEmpty() }
			.joinToString(separator = "\n")

		return if (seedPhrases.isNotBlank()) {
			RepoResult.Success(seedPhrases)
		} else {
			RepoResult.Failure(IllegalStateException("all phrases are empty - broken storage?"))
		}
	}

	suspend fun isSeedPhraseCollision(seedPhrase: String): Boolean {
		return try {
			val result = storage.checkIfSeedNameAlreadyExists(seedPhrase)
			result
		} catch (e: UserNotAuthenticatedException) {
			when (val authResult =
				authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> {
					val result = storage.checkIfSeedNameAlreadyExists(seedPhrase)
					result
				}

				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Timber.e(TAG, "auth error - $authResult")
					false
				}
			}
		}
	}
}


private const val TAG = "Seed_Repository"
//refactor to use OperationResult everywhere?
sealed class RepoResult<T> {
	data class Success<T>(val result: T) : RepoResult<T>()
	data class Failure<T>(val error: Throwable = UnknownError()) : RepoResult<T>()
}

fun <T> RepoResult<T>.toOperationResult(): OperationResult<T,Throwable> {
	return when (this) {
		is RepoResult.Failure -> OperationResult.Err(this.error)
		is RepoResult.Success -> OperationResult.Ok(this.result)
	}
}

fun <T> RepoResult<T>.mapError(): T? {
	return when (this) {
		is RepoResult.Failure -> {
			submitErrorState("uniffi interaction exception $error")
			null
		}

		is RepoResult.Success -> {
			result
		}
	}
}
