package io.parity.signer.domain

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.composed
import androidx.navigation.NavController
import androidx.navigation.NavOptionsBuilder


fun Modifier.conditional(
	condition: Boolean,
	modifier: @Composable Modifier.() -> Modifier
): Modifier = composed {
	if (condition) {
		then(modifier(this))
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


fun NavOptionsBuilder.popUpToTop(navController: NavController) {
	popUpTo(navController.currentBackStackEntry?.destination?.route ?: return) {
		inclusive =  true
	}
}
