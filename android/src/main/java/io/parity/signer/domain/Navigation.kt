package io.parity.signer.domain

import android.content.Context
import android.util.Log
import android.widget.Toast
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.storage.getSeed
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.uniffi.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext


@Deprecated("obsolete, for backwards compatibility, use SignerNavigator class")
fun SharedViewModel.navigate(
	button: Action,
	details: String = "",
	seedPhrase: String = ""
) {
	navigator.navigate(button, details, seedPhrase)
}

interface Navigator {
	/**
	 * For old Rust-backed navigation actions
	 */
	fun navigate(
		action: Action,
		details: String = "",
		seedPhrase: String = ""
	)

	fun backAction()
}


/**
 * Class to navigate within rust state-machine area. It is one (big) part of compose-based navigation.
 * This class have nothing to do with composa-based navigation.
 */
class SignerNavigator() : Navigator {

	private val appContext: Context get() = ServiceLocator.appContext
	private val activity get() = ServiceLocator.activityScope?.activity

	private val _actionResult = MutableStateFlow<ActionResult?>(
		null
	)
	val actionResult: StateFlow<ActionResult?> = _actionResult.asStateFlow()

	private suspend fun navigateRust(
		action: Action,
		details: String = "",
		seedPhrase: String = "",
	): OperationResult<ActionResult, NavigationError> =
		withContext(Dispatchers.IO) {
			try {
				OperationResult.Ok(backendAction(action, details, seedPhrase))
			} catch (e: ErrorDisplayed) {
				OperationResult.Err(
					NavigationError(
						appContext.getString(
							R.string.navigation_error_general_message,
							e.findErrorDisplayed()?.message ?: e.message
						)
					)
				)
			}
		}

	override fun navigate(action: Action, details: String, seedPhrase: String) {

		try {
			val navigationAction = runBlocking {
				val result = navigateRust(action, details, seedPhrase)
				when (result) {
					is OperationResult.Err -> _actionResult.value?.copy(
						alertData = AlertData.ErrorData(result.error.message)
					)

					is OperationResult.Ok -> result.result
				}
			} ?: return

			_actionResult.value = navigationAction
		} catch (e: java.lang.Exception) {
			Log.e("Navigation error", e.toString())
			Toast.makeText(appContext, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}

	override fun backAction() {
		val lastRustNavAction = actionResult.value
		if (lastRustNavAction == null) {
			activity?.moveTaskToBack(true)
		} else if (
			lastRustNavAction.alertData == null &&
			lastRustNavAction.modalData == null &&
			(
				lastRustNavAction.screenData is ScreenData.Log ||
					lastRustNavAction.screenData is ScreenData.Scan ||
					lastRustNavAction.screenData is ScreenData.SeedSelector ||
					lastRustNavAction.screenData is ScreenData.Settings
				)
		) {
			activity?.moveTaskToBack(true)
		} else {
			navigate(Action.GO_BACK)
		}
	}
}

class EmptyNavigator : Navigator {
	override fun navigate(action: Action, details: String, seedPhrase: String) {
		//do nothing
	}

	override fun backAction() {
	}
}

class FakeNavigator : Navigator {
	override fun navigate(action: Action, details: String, seedPhrase: String) {
		try {
			backendAction(action, details, seedPhrase)
		} catch (e: ErrorDisplayed) {
			Log.e("fake navigation error", e.message ?: e.toString())
		}
		//do nothing with result
	}

	override fun backAction() {
		navigate(Action.GO_BACK)
	}
}

data class NavigationError(val message: String)
