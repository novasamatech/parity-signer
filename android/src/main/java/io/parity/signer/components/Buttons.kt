package io.parity.signer.components

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueElement
import io.parity.signer.ui.theme.*

/**
 * Typical huge button that just wants to be pushed
 */
@Composable
fun BigButton(
	text: String,
	isShaded: Boolean = false,
	isCrypto: Boolean = false,
	isDangerous: Boolean = false,
	action: () -> Unit,
	isDisabled: Boolean = false
) {
	val accentColor =
		if (isCrypto) MaterialTheme.colors.Crypto400 else MaterialTheme.colors.Action400
	val bgColor =
		if (isDisabled) MaterialTheme.colors.Bg200 else if (isShaded) MaterialTheme.colors.Bg300 else accentColor
	val fgColor =
		if (isDisabled) MaterialTheme.colors.Text300 else if (isShaded) if (isDangerous) MaterialTheme.colors.SignalDanger else accentColor else MaterialTheme.colors.Action600

	Surface(
		color = bgColor,
		shape = MaterialTheme.shapes.large,
		/*border = BorderStroke(
			1.dp,
			if (isShaded) fgColor else bgColor
		),*/
		modifier = Modifier
            .clickable(onClick = action, enabled = !isDisabled)
            .padding(vertical = 8.dp)
            .height(44.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.Center,
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			Text(text, style = MaterialTheme.typography.button, color = fgColor)
		}
	}
}

/**
 * Small buttons for multiselect screen
 */
@Composable
fun SmallButton(
	text: String,
	isDisabled: Boolean = false,
	action: () -> Unit
) {
	Surface(
		shape = MaterialTheme.shapes.small,
		border = BorderStroke(
			1.dp,
			if (isDisabled) MaterialTheme.colors.Text300 else MaterialTheme.colors.Action400
		),
		color = Color.Transparent,
		modifier = Modifier.clickable(onClick = action, enabled = !isDisabled)
	) {
		Text(
			text,
			style = MaterialTheme.typography.caption,
			color = if (isDisabled) MaterialTheme.colors.Text300 else MaterialTheme.colors.Action400,
			modifier = Modifier.padding(4.dp)
		)
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewOldButtons() {
	Column {
		BigButton("BigButton", action = {})
		SmallButton("SmallButton") {}
	}
}
