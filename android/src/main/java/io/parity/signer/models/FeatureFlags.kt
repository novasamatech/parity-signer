package io.parity.signer.models

object FeatureFlags {
	fun isEnabled(feature: FeatureOption): Boolean {
		return when (feature) {
			FeatureOption.EXPORT_SECRET_KEY -> false
		}
	}
}


enum class FeatureOption { EXPORT_SECRET_KEY }
