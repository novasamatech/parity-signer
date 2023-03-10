package io.parity.signer.components.networkicon

import android.annotation.SuppressLint
import android.content.Context
import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun NetworkIcon(networkName: String) {
	val context = LocalContext.current
	val icon = context.getIconForNetwork(networkName)
	if (icon != null) {
		Image(
			painter = icon,
			contentDescription = null,
			modifier = Modifier
				.size(36.dp)
				.padding(
					top = 16.dp,
					bottom = 16.dp,
					start = 16.dp,
					end = 12.dp
				)
		)
	} else {
		//todo dmitry implement unknown icons
	}
}

@Composable
@SuppressLint("DiscouragedApi")
private fun Context.getIconForNetwork(networkName: String): Painter? {
	val resource = resources.getIdentifier(/* name = */ "network_$networkName",
		/* defType = */
		"drawable",
		/* defPackage = */
		packageName
	)

	return if (resource > 0) {
		painterResource(id = resource)
	} else {
		null
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
private fun PreviewNetworkIcon() {
	SignerNewTheme {
		Column {
			NetworkIcon("polkadot")
			SignerDivider()
			NetworkIcon("some_unknown")
		}
	}
}
