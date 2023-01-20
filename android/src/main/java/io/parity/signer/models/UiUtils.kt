package io.parity.signer.models

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import androidx.compose.ui.Modifier


fun Modifier.conditional(condition : Boolean, modifier : Modifier.() -> Modifier) : Modifier {
	return if (condition) {
		then(modifier(Modifier))
	} else {
		this
	}
}

/**
 * Method, used in accompanist/permission by Google
 */
fun Context.findActivity(): Activity? {
	var context = this
	while (context is ContextWrapper) {
		if (context is Activity) return context
		context = context.baseContext
	}
	return null
}



