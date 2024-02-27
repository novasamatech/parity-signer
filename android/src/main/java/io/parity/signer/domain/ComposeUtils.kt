package io.parity.signer.domain

import android.app.Activity
import android.content.Context
import android.view.WindowManager
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import java.util.concurrent.atomic.AtomicInteger


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
		val count = DisableScreenshotCounter.forbiddenViews.incrementAndGet()
		reactOnCounter(count, context)
		onDispose {
			val counted = DisableScreenshotCounter.forbiddenViews.decrementAndGet()
			reactOnCounter(counted, context)
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
	var forbiddenViews = AtomicInteger(0)
}
