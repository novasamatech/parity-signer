package io.parity.signer.models

object FeatureFlags {
	fun isEnabled(feature: FeatureOption): Boolean {
		return when (feature) {
			FeatureOption.EXPORT_SECRET_KEY -> true
			FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT -> true
		}
	}

	fun isDisabled(feature: FeatureOption): Boolean = !isEnabled(feature)
}


enum class FeatureOption {
	SKIP_UNLOCK_FOR_DEVELOPMENT,
	EXPORT_SECRET_KEY,
}
