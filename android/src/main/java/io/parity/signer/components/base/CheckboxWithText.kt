package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink500
import io.parity.signer.ui.theme.textTertiary

/**
 * Active checkbox field that responds to click anywhere within it
 */
@Composable
fun CheckboxWithText(
	checked: Boolean,
	text: String,
	modifier: Modifier = Modifier,
	onValueChange: (Boolean) -> Unit,
) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = modifier.toggleable(
			value = checked,
			role = Role.Checkbox,
			onValueChange = { onValueChange(it) }
		)
	) {
		CheckboxIcon(
			checked = checked,
		)
		Spacer(Modifier.width(16.dp))
		Text(
			text,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
		)
	}
}

/**
 * Just a checkbox with proper colors
 */
@Composable
fun CheckboxIcon(
	checked: Boolean,
	checkedColor: Color = MaterialTheme.colors.pink500,
	uncheckedColor: Color = MaterialTheme.colors.textTertiary,
	checkmarkColor: Color = Color.White,
) {
	Checkbox(
		checked = checked,
		onCheckedChange = null,
		colors = CheckboxDefaults.colors(
			checkedColor = checkedColor,
			uncheckedColor = uncheckedColor,
			checkmarkColor = checkmarkColor
		)
	)
}

@Composable
fun CheckboxWithTextSecondary(
	checked: Boolean,
	text: String,
	modifier: Modifier = Modifier,
	onValueChange: (Boolean) -> Unit,
) {
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = modifier.toggleable(
			value = checked,
			role = Role.Checkbox,
			onValueChange = { onValueChange(it) }
		)
	) {
		CheckboxIcon(
			checked = checked,
		)
		Spacer(Modifier.width(16.dp))
		Text(
			text,
			color = MaterialTheme.colors.secondary,
			style = SignerTypeface.BodyM,
		)
	}
}

@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewCheckboxWithText() {
	SignerNewTheme {
		Column {
			CheckboxWithText(
				checked = true,
				onValueChange = {},
				text = "Description of this checkbox very long two lines",
			)
			SignerDivider()
			CheckboxWithText(
				checked = false,
				onValueChange = {},
				text = "Description of this checkbox very long two lines",
			)
			SignerDivider()
			SignerDivider()
			CheckboxWithTextSecondary(
				checked = true,
				onValueChange = {},
				text = "Description of this checkbox secondary",
			)
			SignerDivider()
			CheckboxWithTextSecondary(
				checked = false,
				onValueChange = {},
				text = "Description of this checkbox secondary",
			)
		}
	}
}
