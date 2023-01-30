package io.parity.signer.domain

import io.parity.signer.uniffi.ActionResult
import io.parity.signer.uniffi.ScreenData

/**
 * Helper functions that will help compose to skip some composables during redesign
 */
object NavigationMigrations {
	fun shouldShowBar(
		localNavAction: LocalNavAction?,
		globalNavAction: ActionResult?
	): Boolean {

		return when (localNavAction) {
			else -> when (globalNavAction?.screenData) {
				is ScreenData.SeedSelector -> false
				is ScreenData.Keys -> false
				else -> true
			}
		}
	}
}
