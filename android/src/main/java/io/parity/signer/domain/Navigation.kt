package io.parity.signer.domain

import android.content.Context
import android.util.Log
import android.widget.Toast
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.screens.scan.errors.findErrorDisplayed
import io.parity.signer.uniffi.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext


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
