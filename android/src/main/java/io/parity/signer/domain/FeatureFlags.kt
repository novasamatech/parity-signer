package io.parity.signer.domain

import io.parity.signer.BuildConfig

object FeatureFlags {
	fun isEnabled(feature: FeatureOption): Boolean {
		//skip all for release in case it wasn't
		if (!BuildConfig.DEBUG) return false

		return when (feature) {
			FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT -> false
			FeatureOption.SKIP_ROOTED_CHECK_EMULATOR -> false
			FeatureOption.EXPORT_SECRET_KEY -> false //unused
			FeatureOption.FAIL_DB_VERSION_CHECK -> false
			FeatureOption.SKIP_USB_CHECK -> true

		}
	}

	fun isDisabled(feature: FeatureOption): Boolean = !isEnabled(feature)
}


enum class FeatureOption {
	FAIL_DB_VERSION_CHECK,
	SKIP_UNLOCK_FOR_DEVELOPMENT,
	SKIP_ROOTED_CHECK_EMULATOR,
	SKIP_USB_CHECK,
	EXPORT_SECRET_KEY; //unused as sample

	fun isEnabled() = FeatureFlags.isEnabled(this)
}
