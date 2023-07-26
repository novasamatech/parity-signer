package io.parity.signer.bottomsheets

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textTertiary


@Composable
fun PublicKeyBottomSheetView(
	name: String,
	key: String,
	onClose: Callback,
) {
	Column(
		modifier = Modifier.fillMaxWidth(),
	) {
		BottomSheetHeader(
			title = name,
			onCloseClicked = onClose
		)
		SignerDivider(sidePadding = 24.dp)

		Text(
			text = stringResource(R.string.public_key_bottom_sheet_label),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 12.dp, bottom = 8.dp)
		)
		Text(
			text = key,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(bottom = 12.dp)
				.padding(bottom = 16.dp)
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
private fun PreviewPublicKeyBottomSheetView() {
	SignerNewTheme {
		PublicKeyBottomSheetView(
			"Foundation Management",
			"5CfLC887VYVLN6gG5rmp6wyUoXQYVQxEwNekdCbUUphnnQgW",
			{},
		)
	}
}
