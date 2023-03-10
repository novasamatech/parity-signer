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
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun NetworkIcon(networkName: String, modifier: Modifier = Modifier) {
	val icon = getIconForNetwork(networkName)
	if (icon != null) {
		Image(
			painter = icon,
			contentDescription = null,
			modifier = modifier,
		)
	} else {
		//todo dmitry implement unknown icons
	}
}

@Composable
@SuppressLint("DiscouragedApi")
private fun getIconForNetwork(networkName: String): Painter? {
//	val resource = resources.getIdentifier(/* name = */ "network_$networkName",
//		/* defType = */"drawable",/* defPackage = */packageName)

	val id = when (networkName) {
		"polkadot" -> R.drawable.network_polkadot
		else -> -1
	}

	return if (id > 0) {
		painterResource(id = id)
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
			NetworkIcon("some_unknown")
		}
	}
}
