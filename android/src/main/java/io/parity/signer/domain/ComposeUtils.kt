package io.parity.signer.domain

import android.app.Activity
import android.content.Context
import android.view.WindowManager
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView


@Composable
fun KeepScreenOn() {
	val currentView = LocalView.current
	DisposableEffect(Unit) {
		currentView.keepScreenOn = true
		onDispose {
			currentView.keepScreenOn = false
		}
	}
}


private fun Activity.disableScreenshots() {
	window.addFlags(WindowManager.LayoutParams.FLAG_SECURE)
}
private fun Activity.enableScreenshots() {
	window.clearFlags(WindowManager.LayoutParams.FLAG_SECURE)
}

@Composable
fun DisableScreenshots() {
	val context: Context = LocalContext.current
	DisposableEffect(Unit) {
		context.findActivity()?.disableScreenshots()
		onDispose {
			context.findActivity()?.enableScreenshots()
		}
	}
}

