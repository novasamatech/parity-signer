package io.parity.signer.screens

import androidx.compose.material.Text
import androidx.compose.runtime.Composable

/**
 * Settings screen; General purpose stuff like legal info, networks management
 * and history should be here. This is final point in navigation:
 * all subsequent interactions should be in modals or drop-down menus
 */
@Composable
fun SettingsScreen() {
	Text(text = "Settings")
}
