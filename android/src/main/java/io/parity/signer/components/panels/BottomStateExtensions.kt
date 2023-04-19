package io.parity.signer.components.panels

import io.parity.signer.domain.Navigator
import io.parity.signer.uniffi.Action


fun BottomBarState.toAction() =
	when (this) {
		BottomBarState.KEYS -> Action.NAVBAR_KEYS
		BottomBarState.SCANNER -> Action.NAVBAR_SCAN
		BottomBarState.SETTINGS -> Action.NAVBAR_SETTINGS
	}


object CameraParentSingleton {
	/**
	 * Hack to be able to close Camera screens since it's overlay in new design
	 * but still one of bottom sheet states in rust perspective that doesn't back
	 * back action despite it does in new designs
	 */
	var lastPossibleParent: CameraParentScreen = CameraParentScreen.BottomBarScreen(BottomBarState.KEYS)

	fun navigateBackFromCamera(navigator: Navigator) {
		when (val parent = lastPossibleParent) {
			is CameraParentScreen.BottomBarScreen -> {
				navigator.navigate(parent.screen.toAction())
			}
			is CameraParentScreen.NetworkDetailsScreen -> {
				navigator.navigate(Action.NAVBAR_SETTINGS)
				navigator.navigate(Action.MANAGE_NETWORKS)
				navigator.navigate(Action.GO_FORWARD, parent.networkKey)
			}
			CameraParentScreen.NetworkListScreen -> {
				navigator.navigate(Action.NAVBAR_SETTINGS)
				navigator.navigate(Action.MANAGE_NETWORKS)
			}
		}
	}
}

sealed class CameraParentScreen() {
	data class BottomBarScreen(val screen: BottomBarState) : CameraParentScreen()
	object NetworkListScreen : CameraParentScreen()
	data class NetworkDetailsScreen(val networkKey: String) : CameraParentScreen()
}
