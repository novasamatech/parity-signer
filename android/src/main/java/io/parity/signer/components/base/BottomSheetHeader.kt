package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.annotation.StringRes
import androidx.compose.foundation.layout.*
import androidx.compose.material.Divider
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.textSecondary

@Composable
fun BottomSheetHeader(
	title: String,
	subtitile: String? = null,
	onCloseClicked: () -> Unit
) {
	Row(
		modifier = Modifier
			.padding(top = 20.dp, bottom = 20.dp, start = 24.dp, end = 16.dp)
			.fillMaxWidth(),
		verticalAlignment = Alignment.CenterVertically,
	) {
		Column() {
			Text(
				text = title,
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.TitleS,
			)
			//todo dmitry if subtitle paddings should be smaller
			if (subtitile != null) {
				Text(
					text = subtitile,
					color = MaterialTheme.colors.textSecondary,
					style = TypefaceNew.BodyM,
				)
			}
		}
		Spacer(modifier = Modifier.weight(1.0f))
		CloseIcon(onCloseClicked = onCloseClicked)
	}
}

//todo dmitry finish
@Composable
fun BottomSheetSubtitle(@StringRes id: Int) {
	Text(
		text = stringResource(id),
		color = MaterialTheme.colors.primary,
		style = TypefaceNew.BodyL,
		modifier = Modifier.padding(vertical = 6.dp)
	)
}

@Preview(
	name = "day",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,
	backgroundColor = 0xFFFFFFFF
)
@Preview(
	name = "dark theme",
	group = "themes",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true,
	backgroundColor = 0xFF000000
)
@Composable
private fun PreviewHeaderWithClose() {
	SignerNewTheme {
		Column() {
			BottomSheetHeader(title = "Title") {}
			Divider()
			BottomSheetHeader(title = "Title", subtitile = "With subtitle") {}
			Divider()
			BottomSheetSubtitle(R.string.subtitle_secret_recovery_phrase)
		}
	}
}
