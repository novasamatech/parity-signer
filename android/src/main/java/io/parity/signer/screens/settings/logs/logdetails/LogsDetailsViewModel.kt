package io.parity.signer.screens.settings.logs.logdetails

import timber.log.Timber
import androidx.lifecycle.ViewModel
import io.parity.signer.domain.backend.CompletableResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.uniffi.MLogDetails
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext

/**
 * Entity that syncs Rust backend with current UI requests
 */
class LogsDetailsViewModel(): ViewModel() {
	val uniffiInteractor = ServiceLocator.uniffiInteractor

	private val _logsState: MutableStateFlow<CompletableResult<MLogDetails, String>> =
        MutableStateFlow(CompletableResult.InProgress)
	val logsState: StateFlow<CompletableResult<MLogDetails, String>> = _logsState.asStateFlow()

	suspend fun updateLogDetails(index: UInt) {
		when (val result = withContext(Dispatchers.IO) {
			uniffiInteractor.getLogDetails(index)
		}) {
			is UniffiResult.Error -> {
				val error = result.error.getDebugDetailedDescriptionString()
                Timber.e(TAG, "Unexpected error getLogs, $error")
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
}
private const val TAG = "LogsDetailsViewModel"
