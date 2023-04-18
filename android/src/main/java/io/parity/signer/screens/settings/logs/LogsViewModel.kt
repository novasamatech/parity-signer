package io.parity.signer.screens.settings.logs

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.CompletableResult
import io.parity.signer.backend.OperationResult
import io.parity.signer.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.DispatchersRustSingle
import io.parity.signer.domain.getDetailedDescriptionString
import io.parity.signer.uniffi.MLog
import io.parity.signer.uniffi.MLogDetails
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext


/**
 * Entity that syncs Rust backend with current UI requests
 */
class LogsViewModel(): ViewModel() {
	val uniffiInteractor = ServiceLocator.uniffiInteractor

	private val _logsState: MutableStateFlow<CompletableResult<MLog, String>> =
		MutableStateFlow(CompletableResult.InProgress)
	val logsState: StateFlow<CompletableResult<MLog, String>> = _logsState.asStateFlow()

	suspend fun updateLogsData() {
		when (val result = withContext(DispatchersRustSingle) { uniffiInteractor.getLogs() }) {
			is UniffiResult.Error -> {
				val error = result.error.getDetailedDescriptionString()
				Log.e(TAG, "Unexpected error getLogs, $error")
				_logsState.value = CompletableResult.Err(error)
			}
			is UniffiResult.Success -> {
				_logsState.value  = CompletableResult.Ok(result.result)
			}
		}
	}

	fun resetValues() {
		_logsState.value = CompletableResult.InProgress
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
