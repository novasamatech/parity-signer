package io.parity.signer

/**
 * List all possible buttons for typesafe navigation
 */
enum class ButtonID {
	NavbarLog,
	NavbarScan,
	NavbarKeys,
	NavbarSettings,
	GoBack;

	fun getName(): String {
		return when(this) {
			NavbarLog -> "Log"
			NavbarScan -> "Scan"
			NavbarKeys -> "Keys"
			NavbarSettings -> "Settings"
			GoBack -> "<"
		}
	}
}
