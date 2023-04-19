package io.parity.signer.domain.storage

import android.security.keystore.UserNotAuthenticatedException
import android.util.Log
import android.widget.Toast
import androidx.fragment.app.FragmentActivity
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.Authentication
import io.parity.signer.domain.Navigator
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.updateSeedNames
import io.parity.signer.domain.submitErrorState


class SeedRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
) {

	fun containSeedName(seedName: String) : Boolean {
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
			Log.d("get seed failure", e.toString())
			Toast.makeText(activity, "get seed failure: $e", Toast.LENGTH_LONG).show()
			RepoResult.Failure(RuntimeException("Unexpected Exception", e))
		}
	}

	/**
	 * Force ask for authentication and get seed phrase
	 */
	suspend fun getSeedPhraseForceAuth(seed: String): RepoResult<String> {
		return when (val authResult =
			authentication.authenticate(activity)) {
			AuthResult.AuthSuccess -> {
				getSeedPhrasesDangerous(listOf(seed))
			}
			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				RepoResult.Failure(RuntimeException("auth error - $authResult"))
			}
		}
	}


	/**
	 * Add seed, encrypt it, and create default accounts
	 *
	 * @return if was successfully added
	 */
	suspend fun addSeed(
		seedName: String,
		seedPhrase: String,
		navigator: Navigator,
		isOptionalAuth: Boolean,
	): Boolean {
		// Check if seed name already exists
		if (isSeedPhraseCollision(seedPhrase)) {
			return false
		}

		try {
			if (isOptionalAuth) {
				addSeedDangerous(seedName, seedPhrase, navigator)
				return true
			} else {
				throw UserNotAuthenticatedException()
			}
		} catch (e: UserNotAuthenticatedException) {
			return when (val authResult = authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> {
					addSeedDangerous(seedName, seedPhrase, navigator)
					true
				}
				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Log.e(TAG, "auth error - $authResult")
					false
				}
			}
		} catch (e: java.lang.Exception) {
			Log.e(TAG, e.toString())
			return false
		}
	}

	private fun addSeedDangerous(
		seedName: String,
		seedPhrase: String,
		navigator: Navigator
	) {
		storage.addSeed(seedName, seedPhrase)
		tellRustSeedNames()
		val alwaysCreateRoots = "true"
		navigator.navigate(
			action = Action.GO_FORWARD,
			details = alwaysCreateRoots,
			seedPhrase = seedPhrase
		)
	}

	internal fun tellRustSeedNames() {
		val allNames = storage.getSeedNames()
		updateSeedNames(allNames.toList())
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
					Log.e(TAG, "auth error - $authResult")
					false
				}
			}
		}
	}
}


private const val TAG = "Seed_Repository"

sealed class RepoResult<T> {
	data class Success<T>(val result: T) : RepoResult<T>()
	data class Failure<T>(val error: Throwable = UnknownError()) : RepoResult<T>()
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
