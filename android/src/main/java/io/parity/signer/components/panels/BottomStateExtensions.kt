package io.parity.signer.components.panels

import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.FooterButton


fun BottomBar2State.toAction() =
	when (this) {
		BottomBar2State.KEYS -> Action.NAVBAR_KEYS
		BottomBar2State.SCANNER -> Action.NAVBAR_SCAN
		BottomBar2State.LOGS -> Action.NAVBAR_LOG
		BottomBar2State.SETTINGS -> Action.NAVBAR_SETTINGS
	}


object BottomBarSingleton {
	/**
	 * Hack to be able to close Camera screens since it's overlay in new design
	 * but still one of bottom sheet states in rust perspective that doesn't back
	 * back action despite it does in new designs
	 */
	var lastUsedTab: BottomBar2State = BottomBar2State.KEYS
}
