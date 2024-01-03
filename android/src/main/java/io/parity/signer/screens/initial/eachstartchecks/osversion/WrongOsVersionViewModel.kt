package io.parity.signer.screens.initial.eachstartchecks.osversion

import android.os.Build
import androidx.lifecycle.ViewModel


class WrongOsVersionViewModel() : ViewModel() {

	fun isShouldShow(): Boolean {
		return Build.VERSION.SDK_INT < MinRecommendedOsVersion
	}

	fun getCurrentOSVersion(): String {
		return Build.VERSION.RELEASE
	}

	fun getMinRecommendedOsVersion(): String {
		return MinRecommendedOsVersionString
	}
}

//todo dmitry set values given my security team
const val MinRecommendedOsVersion: Int = Build.VERSION_CODES.O
const val MinRecommendedOsVersionString: String = "Oreo"
