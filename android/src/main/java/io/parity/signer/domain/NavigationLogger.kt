package io.parity.signer.domain

import android.os.Bundle
import timber.log.Timber
import androidx.navigation.NavController
import androidx.navigation.NavDestination
import androidx.navigation.NavHostController
import io.parity.signer.BuildConfig


fun NavHostController.addVaultLogger(tag: String = "Navigation") {
	if (BuildConfig.DEBUG) addOnDestinationChangedListener(NavLogger(tag))
}

private class NavLogger(val tag: String) :
	NavController.OnDestinationChangedListener {
	override fun onDestinationChanged(
		controller: NavController,
		destination: NavDestination,
		arguments: Bundle?
	) {
		Timber.d(tag, "destination is " + destination.route)
	}

}
