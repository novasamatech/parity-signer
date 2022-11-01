package io.parity.signer.models

object FeatureFlags {
	fun isEnabled(feature: FeatureOption): Boolean {
		return when (feature) {
			FeatureOption.EXPORT_SECRET_KEY -> true
		}
	}

	fun isDisabled(feature: FeatureOption): Boolean = !isEnabled(feature)
}


enum class FeatureOption {
	EXPORT_SECRET_KEY,
}
