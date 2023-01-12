package io.parity.signer.models

import io.parity.signer.BuildConfig

object FeatureFlags {
	fun isEnabled(feature: FeatureOption): Boolean {
		//skip all for release in case it wasn't
		if (!BuildConfig.DEBUG) return false

		return when (feature) {
			FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT -> false
			FeatureOption.EXPORT_SECRET_KEY -> false //unused
			FeatureOption.NEW_DERIVATION -> true
		}
	}

	fun isDisabled(feature: FeatureOption): Boolean = !isEnabled(feature)
}


enum class FeatureOption {
	NEW_DERIVATION,
	SKIP_UNLOCK_FOR_DEVELOPMENT,
	EXPORT_SECRET_KEY; //unused as sample

	fun isEnabled() = FeatureFlags.isEnabled(this)
}
