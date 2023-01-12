package io.parity.signer.models.storage

import android.security.keystore.UserNotAuthenticatedException
import android.util.Log
import android.widget.Toast
import androidx.fragment.app.FragmentActivity
import io.parity.signer.backend.UniffiResult
import io.parity.signer.models.AuthResult
import io.parity.signer.models.Authentication
import io.parity.signer.models.submitErrorState


class SeedRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
) {

	/**
	 * Try to get phrases if timeout if
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
}

sealed class RepoResult<T> {
	data class Success<T>(val result: T) : RepoResult<T>()
	data class Failure<T>(val error: Throwable) : RepoResult<T>()
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
