package io.parity.signer.components

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.textSecondary


@Composable
fun NetworkLabelWithIcon(networkName: String,
												 networkLogo: String,
												 iconHeight: Dp = 24.dp,
												 style: TextStyle = SignerTypeface.BodyM) {
	Row(
		modifier = Modifier
			.background(
				MaterialTheme.colors.fill12, RoundedCornerShape(iconHeight * 2)
			)
			.padding(start = 2.dp, top = 2.dp, bottom = 2.dp)
			.height(iconHeight),
		verticalAlignment = Alignment.CenterVertically,
	) {
		NetworkIcon(networkLogo, size = iconHeight)
		Text(
			networkName,
			color = MaterialTheme.colors.textSecondary,
			style = style,
			modifier = Modifier
				.padding(start = 6.dp, end = iconHeight / 3)
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
private fun PreviewNetworkLabelWithIcon() {
	SignerNewTheme {
		Column {
			NetworkLabelWithIcon("kusama", "kusama", 24.dp, SignerTypeface.BodyM)
			SignerDivider()
			NetworkLabelWithIcon("kusama", "", 24.dp, SignerTypeface.BodyM)
		}
	}
}
