package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary

@Composable
fun TCNameValueElement(
	name: String? = null,
	value: String? = null,
	valueInSameLine: Boolean = true,
) {
	if (valueInSameLine) {
		Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
			if (name?.isNotEmpty() == true) {
				Text(
					text = name,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textTertiary
				)
			}
			if (value?.isNotEmpty() == true) {
				Text(
					text = value,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary
				)
			}
		}
	} else {
		Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
			if (name?.isNotEmpty() == true) {
				Text(
					text = name,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textTertiary
				)
			}
			if (value?.isNotEmpty() == true) {
				Text(
					text = value,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary
				)
			}
		}
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
private fun PreviewTCNameValueElement() {
	SignerNewTheme {
		Column {
			TCNameValueElement(
				name = "Name",
				value = "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq"
			)
			SignerDivider()
			TCNameValueElement(name = "Name", value = "value")
			SignerDivider()
			TCNameValueElement(
				name = "Name",
				value = "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
				valueInSameLine = false
			)
		}
	}
}
