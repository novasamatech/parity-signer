package io.parity.signer.domain.usecases

import io.parity.signer.domain.FeatureFlags
import io.parity.signer.domain.FeatureOption
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.uniffi.ErrorDisplayed
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext


class DBVersionValidationUseCase {

	suspend fun validate(): UniffiResult<Unit> = withContext(Dispatchers.IO) {
		if (FeatureFlags.isEnabled(FeatureOption.FAIL_DB_VERSION_CHECK)) {
			return@withContext UniffiResult.Error(ErrorDisplayed.DbSchemaMismatch())
		}
		try {
			io.parity.signer.uniffi.checkDbVersion()
			UniffiResult.Success(Unit)
		} catch (e: ErrorDisplayed) {
			UniffiResult.Error(e)
		}
	}
}
