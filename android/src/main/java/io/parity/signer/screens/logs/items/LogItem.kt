package io.parity.signer.screens.logs.items

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.textTertiary

@Composable
fun LogItem(
	title: String,
	message: String,
	time: String,
) {
	Column(
        Modifier
            .fillMaxWidth(1f)
            .padding(vertical = 8.dp, horizontal = 24.dp)
	) {
		Text(
			text = title,
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.TitleS,
		)
		Spacer(modifier = Modifier.padding(top = 8.dp))
		Row(
			verticalAlignment = Alignment.CenterVertically,
		) {
			Text(
				text = message,
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
				modifier = Modifier.weight(1f)
			)
			Spacer(modifier = Modifier.padding(start = 8.dp))
			Text(
				text = time,
				color = MaterialTheme.colors.textTertiary,
				style = TypefaceNew.BodyM,
			)
			Image(
				imageVector = Icons.Filled.ChevronRight,
				contentDescription = null,
				colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
//				modifier = Modifier
//					.size(28.dp)
//					.padding(end = 8.dp)
			)
		}
	}
}

@Composable
fun LogItemDate(date: String) {
	Text(
		text = date,
		color = MaterialTheme.colors.textTertiary,
		style = TypefaceNew.BodyM,
		modifier = Modifier.padding(vertical = 8.dp, horizontal = 24.dp)
	)
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
private fun PreviewLogItem() {
	SignerNewTheme {
		Column() {
			LogItemDate(date = "Jun 20")
			LogItem(
				title = "Database initiated",
				message = "Message of database init happened very long 2 lines",
				time = "10:42",
			)

		}
	}
}
