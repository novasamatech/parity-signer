package io.parity.signer.components

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material.Checkbox
import androidx.compose.material.CheckboxDefaults
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Action600

/**
 * Active checkbox field that responds to click anywhere within it
 */
@Composable
fun CheckboxTemplate(
	checked: Boolean,
	onValueChange: (Boolean) -> Unit,
	text: String
) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.toggleable(
			value = checked,
			role = Role.Checkbox,
			onValueChange = { onValueChange(it) }
		)
	) {
		CheckboxIcon(
			checked = checked,
		)
		Spacer(Modifier.width(8.dp))
		Text(
			text,
			style = MaterialTheme.typography.body1,
			color = MaterialTheme.colors.Action400
		)
	}
}

/**
 * Just a checkbox with proper colors
 */
@Composable
fun CheckboxIcon(
	checked: Boolean
) {
	Checkbox(
		checked = checked,
		onCheckedChange = null,
		colors = CheckboxDefaults.colors(
			checkedColor = MaterialTheme.colors.Action400,
			uncheckedColor = MaterialTheme.colors.Action400,
			checkmarkColor = MaterialTheme.colors.Action600
		)
	)
}
