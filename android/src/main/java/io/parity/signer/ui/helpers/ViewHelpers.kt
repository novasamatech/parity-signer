package io.parity.signer.ui.helpers

import android.view.View
import android.view.ViewTreeObserver


inline fun View.afterMeasured(crossinline block: () -> Unit) {
	if (measuredWidth > 0 && measuredHeight > 0) {
		block()
	} else {
		viewTreeObserver.addOnGlobalLayoutListener(object : ViewTreeObserver.OnGlobalLayoutListener {
			override fun onGlobalLayout() {
				if (measuredWidth > 0 && measuredHeight > 0) {
					viewTreeObserver.removeOnGlobalLayoutListener(this)
					block()
				}
			}
		})
	}
}
