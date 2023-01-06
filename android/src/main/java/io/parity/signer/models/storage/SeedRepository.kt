package io.parity.signer.models.storage

import android.security.keystore.UserNotAuthenticatedException
import android.util.Log
import android.widget.Toast
import androidx.fragment.app.FragmentActivity
import io.parity.signer.models.AuthResult
import io.parity.signer.models.Authentication


class SeedRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
) {

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

