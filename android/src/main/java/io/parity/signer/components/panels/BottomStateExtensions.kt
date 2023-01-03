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

fun FooterButton.toBottomBarState(): BottomBar2State? =
	when (this) {
		FooterButton.LOG -> BottomBar2State.LOGS
		FooterButton.KEYS -> BottomBar2State.KEYS
		FooterButton.SETTINGS -> BottomBar2State.SETTINGS
		FooterButton.SCAN -> null
		FooterButton.BACK -> null
	}


object BottomBarSingleton {
	var lastUsedTab: BottomBar2State = BottomBar2State.KEYS
}
