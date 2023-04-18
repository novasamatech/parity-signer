package io.parity.signer.screens.settings.logs

import android.util.Log
import io.parity.signer.backend.OperationResult
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.DispatchersRustSingle
import io.parity.signer.domain.getDetailedDescriptionString
import io.parity.signer.uniffi.MLog
import io.parity.signer.uniffi.MLogDetails
import kotlinx.coroutines.withContext


/**
 * Entity that syncs Rust backend with current UI requests
 */
class LogsViewModel() {
	val uniffiInteractor = ServiceLocator.uniffiInteractor

	suspend fun getLogsData(): OperationResult<MLog, String> {
		return when (val result = withContext(DispatchersRustSingle) { uniffiInteractor.getLogs() }) {
			is UniffiResult.Error -> {
				val error = result.error.getDetailedDescriptionString()
				Log.e(TAG, "Unexpected error getLogs, $error")
				OperationResult.Err(error)
			}
			is UniffiResult.Success -> {
				OperationResult.Ok(result.result)
			}
		}
	}

	suspend fun getLogDetails(logIndex: UInt): OperationResult<MLogDetails, String> {
		return when (val result = withContext(DispatchersRustSingle) { uniffiInteractor.getLogDetails(logIndex) }) {
			is UniffiResult.Error -> {
				val error = result.error.getDetailedDescriptionString()
				Log.e(TAG, "Unexpected error getLogs, $error")
				OperationResult.Err(error)
			}
			is UniffiResult.Success -> {
				OperationResult.Ok(result.result)
			}
		}
	}
}

private const val TAG = "LogsViewModel"
