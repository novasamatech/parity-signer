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

/**
 * We need counter here since during navigation new view disposable effect start
 * before old one onDispose(). And we want screenshots still be forbidden if both
 * screens forbids it
 */
@Composable
fun DisableScreenshots() {
	val context: Context = LocalContext.current
	DisposableEffect(Unit) {
		DisableScreenshotCounter.counter ++
		reactOnCounter(DisableScreenshotCounter.counter, context)
		onDispose {
			DisableScreenshotCounter.counter --
			reactOnCounter(DisableScreenshotCounter.counter, context)
		}
	}
}

private fun reactOnCounter(counter: Int, context: Context) {
 if (counter > 0) {
	 context.findActivity()?.disableScreenshots()
 } else {
	 context.findActivity()?.enableScreenshots()
 }
}

private object DisableScreenshotCounter {
	var counter: Int = 0 //not synced as always interacted from main thread
}
