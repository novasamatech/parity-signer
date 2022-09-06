package io.parity.signer.components

import androidx.compose.material.*
import androidx.compose.runtime.Composable

/**
 * Alert component used everywhere in Signer
 */
@Composable
fun AlertComponent(
	show: Boolean,
	header: String = "Are you sure?",
	text: String? = null, //null is preferred for UX
	back: () -> Unit,
	forward: () -> Unit,
	backText: String = "Cancel",
	forwardText: String = "Confirm",
	showForward: Boolean = true,
	) {
		if (show) {
			AlertDialog(
				onDismissRequest = back,
				dismissButton = {
					Button(
						colors = ButtonDefaults.buttonColors(
							backgroundColor = MaterialTheme.colors.background,
							contentColor = MaterialTheme.colors.onBackground,
						),
						onClick = back
					) {
						Text(backText)
					}
				},
				confirmButton = {
					if (showForward) {
						Button(
							colors = ButtonDefaults.buttonColors(
								backgroundColor = MaterialTheme.colors.background,
								contentColor = MaterialTheme.colors.onBackground,
							),
							onClick = forward
						) {
							Text(forwardText)
						}
					}
				},
				title = { Text(header) },
				text = { text?.let { Text(it) } }
			)
		}
	}
