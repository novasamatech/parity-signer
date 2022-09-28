package io.parity.signer.models

import io.parity.signer.uniffi.ActionResult

/**
 * Helper functions that will help compose to skip some composables during redesign
 */
object NavigationMigrations {
	fun shouldShowTopBar(localNavAction: LocalNavAction?,
											 globalNavAction: ActionResult?): Boolean {

		return true
	}
}
