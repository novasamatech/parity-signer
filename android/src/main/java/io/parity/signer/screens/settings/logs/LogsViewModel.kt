package io.parity.signer.screens.settings.logs

import android.content.Context
import timber.log.Timber
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.parity.signer.domain.backend.CompletableResult
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.UniffiResult
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.findActivity
import io.parity.signer.domain.getDebugDetailedDescriptionString
import io.parity.signer.uniffi.MLog
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext


/**
 * Entity that syncs Rust backend with current UI requests
 */
class LogsViewModel() : ViewModel() {
	val uniffiInteractor = ServiceLocator.uniffiInteractor

	private val _logsState: MutableStateFlow<CompletableResult<MLog, String>> =
		MutableStateFlow(CompletableResult.InProgress)
	val logsState: StateFlow<CompletableResult<MLog, String>> =
		_logsState.asStateFlow()

	suspend fun updateLogsData() {
		when (val result =
			withContext(Dispatchers.IO) { uniffiInteractor.getLogs() }) {
			is UniffiResult.Error -> {
				val error = result.error.getDebugDetailedDescriptionString()
				Timber.e(TAG, "Unexpected error getLogs, $error")
				_logsState.value = CompletableResult.Err(error)
			}
			is UniffiResult.Success -> {
				_logsState.value = CompletableResult.Ok(result.result)
			}
		}
	}

	suspend fun addLogNote(logNote: String): OperationResult<Unit, String> {
		return when (val result =
			withContext(Dispatchers.IO) { uniffiInteractor.addCommentToLogs(logNote) }) {
			is UniffiResult.Error -> {
				val error = result.error.getDebugDetailedDescriptionString()
				Timber.e(TAG, "Unexpected error addNote, $error")
				OperationResult.Err(error)
			}
			is UniffiResult.Success -> {
				OperationResult.Ok(result.result)
			}
		}
	}

	fun actionClearLogsHistory(context: Context) {
		viewModelScope.launch {
			val authenticator = ServiceLocator.authentication
			when (authenticator.authenticate(context.findActivity() as FragmentActivity)) {
				AuthResult.AuthSuccess -> {
					clearLogHistory()
					updateLogsData()
				}
				AuthResult.AuthError,
				AuthResult.AuthFailed,
				AuthResult.AuthUnavailable -> {
					Timber.d("Vault", "Can't remove logs without authentication")
				}
			}
		}
	}
	private suspend fun clearLogHistory(): OperationResult<Unit, String> {
		return when (val result =
			withContext(Dispatchers.IO) { uniffiInteractor.clearLogHistory() }) {
			is UniffiResult.Error -> {
				val error = result.error.getDebugDetailedDescriptionString()
				Timber.e(TAG, "Unexpected error clear logs, $error")
				OperationResult.Err(error)
			}
			is UniffiResult.Success -> {
				OperationResult.Ok(result.result)
			}
		}
	}

	fun resetValues() {
		_logsState.value = CompletableResult.InProgress
	}
}

private const val TAG = "LogsViewModel"
