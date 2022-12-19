package io.parity.signer.models.storage

import androidx.fragment.app.FragmentActivity
import io.parity.signer.models.AuthResult
import io.parity.signer.models.Authentication


class SeedRepository(
	private val storage: SeedStorage,
	private val authentication: Authentication,
	private val activity: FragmentActivity,
) {

	suspend fun getSeedPhrases(seedNames: List<String>): RepoResult<String> {
		try {
			return when (val authResult =
				authentication.authenticate(activity)) {
				AuthResult.AuthSuccess -> {
					val seedPhrases = seedNames
						.map { storage.getSeed(it) }
						.filter { it.isNotEmpty() }
						.joinToString(separator = "\n")

					if (seedPhrases.isNotBlank()) {
						RepoResult.Success(seedPhrases)
					} else {
						RepoResult.Failure(IllegalStateException("all phrases are empty - broken storage?"))
					}
				}
				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					RepoResult.Failure(RuntimeException("auth error - $authResult"))
				}
			}
		} catch (e: java.lang.Exception) {
			return RepoResult.Failure(RuntimeException("Unexpected Exception", e))
		}
	}
}

sealed class RepoResult<T> {
	data class Success<T>(val result: T) : RepoResult<T>()
	data class Failure<T>(val error: Throwable) : RepoResult<T>()
}

