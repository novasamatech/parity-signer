package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun BottomSheetHeader(header: String, onCancelClicked: () -> Unit) {
	Row(
		modifier = Modifier
			.padding(top = 20.dp, bottom = 20.dp, start = 24.dp, end = 16.dp)
			.fillMaxWidth(),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Text(
			text = header,
			color = MaterialTheme.colors.primary,
			style = MaterialTheme.typography.h3,
		)
		Spacer(modifier = Modifier.weight(1.0f))
		CloseIcon(onCloseClicked = onCancelClicked)
	}
}




@Preview(
	name = "day",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
	backgroundColor = 0xFFFFFFFF
)
@Preview(
	name = "dark theme", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000
)
@Composable
private fun PreviewHeaderWithClose() {
	SignerNewTheme() {
		BottomSheetHeader(header = "Title") {

		}
	}
}
