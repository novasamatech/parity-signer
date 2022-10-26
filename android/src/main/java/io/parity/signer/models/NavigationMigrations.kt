package io.parity.signer.models

import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.ScreenData

/**
 * Helper functions that will help compose to skip some composables during redesign
 */
object NavigationMigrations {
	fun shouldShowBar(localNavAction: LocalNavAction?,
										globalNavAction: ActionResult?): Boolean {

		return when (globalNavAction?.screenData) {
			is ScreenData.SeedSelector -> false;
			is ScreenData.Keys -> FeatureFlags.isDisabled(FeatureOption.NEW_KEY_SET_DETAILS)
			else ->  true
		}
	}
}
